package com.faiq.companion.data

import android.content.Context
import com.faiq.companion.model.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import org.json.JSONArray
import org.json.JSONObject
import java.text.SimpleDateFormat
import java.util.*
import java.util.concurrent.TimeUnit

class SyncRepository(private val context: Context) {
    private val client = OkHttpClient.Builder()
        .connectTimeout(4, TimeUnit.SECONDS)
        .readTimeout(6, TimeUnit.SECONDS)
        .build()

    private val prefs = context.getSharedPreferences("faiq_prefs", Context.MODE_PRIVATE)

    var laptopHost: String
        get() = prefs.getString("laptop_host", "10.0.2.2:4242") ?: "10.0.2.2:4242"
        set(value) = prefs.edit().putString("laptop_host", value).apply()

    private val queuedMessages = mutableListOf<OfflineMessage>()
    private val queuedNotes = mutableListOf<String>()

    var currentState: FaiqState = FaiqState()
        private set

    var activeGoals: List<TacticalGoal> = emptyList()
        private set

    var topLessons: List<Lesson> = emptyList()
        private set

    var lastSyncStatus: String = "Not synced yet"
        private set

    fun queueNote(note: String) {
        queuedNotes.add(note)
    }

    fun getQueuedNotesCount(): Int = queuedNotes.size
    fun getQueuedMessagesCount(): Int = queuedMessages.size

    suspend fun syncWithLaptop(): Result<SyncPushResponse> = withContext(Dispatchers.IO) {
        try {
            val url = "http://$laptopHost/api/sync/push"

            val jsonBody = JSONObject().apply {
                put("device_id", "android-companion")
                put("offline_messages", JSONArray().apply {
                    queuedMessages.forEach { msg ->
                        put(JSONObject().apply {
                            put("timestamp_rfc3339", msg.timestampRfc3339)
                            put("user_prompt", msg.userPrompt)
                            if (msg.response != null) put("response", msg.response)
                        })
                    }
                })
                put("offline_notes", JSONArray().apply {
                    queuedNotes.forEach { put(it) }
                })
            }

            val request = Request.Builder()
                .url(url)
                .post(jsonBody.toString().toRequestBody("application/json".toMediaType()))
                .build()

            val response = client.newCall(request).execute()
            if (!response.isSuccessful) {
                lastSyncStatus = "Sync failed (HTTP ${response.code})"
                return@withContext Result.failure(Exception("Sync failed: HTTP ${response.code}"))
            }

            val respJson = JSONObject(response.body?.string() ?: "{}")
            val accepted = respJson.optInt("accepted_events", 0)
            val xpAwarded = respJson.optDouble("xp_awarded", 0.0)

            // Parse updated state
            val stateObj = respJson.optJSONObject("updated_state")
            if (stateObj != null) {
                val traitsObj = stateObj.optJSONObject("traits")
                val levelObj = stateObj.optJSONObject("level_engine")
                val innerObj = stateObj.optJSONObject("inner_state")

                currentState = FaiqState(
                    traits = NumericTraits(
                        confidence = traitsObj?.optDouble("confidence", 80.0) ?: 80.0,
                        resilience = traitsObj?.optDouble("resilience", 90.0) ?: 90.0,
                        intelligence = traitsObj?.optDouble("intelligence", 85.0) ?: 85.0,
                        calculation = traitsObj?.optDouble("calculation", 92.0) ?: 92.0,
                        courage = traitsObj?.optDouble("courage", 88.0) ?: 88.0
                    ),
                    innerState = InnerState(
                        stressLevel = innerObj?.optDouble("stress_level", 0.1) ?: 0.1,
                        focus = innerObj?.optDouble("focus", 0.9) ?: 0.9,
                        urgency = innerObj?.optDouble("urgency", 0.2) ?: 0.2
                    ),
                    levelEngine = LevelEngine(
                        level = levelObj?.optInt("level", 1) ?: 1,
                        xp = levelObj?.optDouble("xp", 0.0) ?: 0.0,
                        totalExperiences = levelObj?.optLong("total_experiences", 0) ?: 0
                    )
                )
            }

            // Parse top lessons
            val lessonsArray = respJson.optJSONArray("top_lessons")
            val lessons = mutableListOf<Lesson>()
            if (lessonsArray != null) {
                for (i in 0 until lessonsArray.length()) {
                    val l = lessonsArray.getJSONObject(i)
                    lessons.add(
                        Lesson(
                            id = l.optLong("id"),
                            principle = l.optString("principle", ""),
                            context = l.optString("context", ""),
                            relevanceCount = l.optLong("relevance_count", 1)
                        )
                    )
                }
            }
            topLessons = lessons

            // Clear synced queues upon success
            queuedMessages.clear()
            queuedNotes.clear()
            lastSyncStatus = "Synced ($accepted events merged, +${String.format("%.1f", xpAwarded)} XP)"

            Result.success(
                SyncPushResponse(
                    acceptedEvents = accepted,
                    xpAwarded = xpAwarded,
                    state = currentState,
                    activeGoals = activeGoals,
                    topLessons = topLessons
                )
            )
        } catch (e: Exception) {
            lastSyncStatus = "Offline (${e.localizedMessage ?: "Connection refused"})"
            Result.failure(e)
        }
    }

    suspend fun sendChatMessage(userMessage: String): ApiChatResponse = withContext(Dispatchers.IO) {
        val nowRfc = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'", Locale.US).apply {
            timeZone = TimeZone.getTimeZone("UTC")
        }.format(Date())

        try {
            val url = "http://$laptopHost/api/chat"
            val jsonBody = JSONObject().apply {
                put("message", userMessage)
            }

            val request = Request.Builder()
                .url(url)
                .post(jsonBody.toString().toRequestBody("application/json".toMediaType()))
                .build()

            val response = client.newCall(request).execute()
            if (response.isSuccessful) {
                val respJson = JSONObject(response.body?.string() ?: "{}")
                return@withContext ApiChatResponse(
                    reply = respJson.optString("reply", "Acknowledged."),
                    level = respJson.optInt("level", currentState.levelEngine.level),
                    xp = respJson.optDouble("xp", currentState.levelEngine.xp),
                    composure = respJson.optDouble("composure", 0.99),
                    executionClarity = respJson.optDouble("execution_clarity", 0.91),
                    directive = respJson.optString("directive", "Maintain total objective composure.")
                )
            }
        } catch (_: Exception) {
            // Daemon is unreachable: queue locally and provide immediate calculated response
        }

        // Offline fallback & queueing
        val offlineReply = "Offline mode active. Your message has been queued locally. Faiq will process, reflect, and award XP once reconnected to your laptop daemon."
        queuedMessages.add(
            OfflineMessage(
                timestampRfc3339 = nowRfc,
                userPrompt = userMessage,
                response = offlineReply
            )
        )
        lastSyncStatus = "Offline (${queuedMessages.size} messages queued)"

        ApiChatResponse(
            reply = offlineReply,
            level = currentState.levelEngine.level,
            xp = currentState.levelEngine.xp,
            composure = 1.0 - (currentState.innerState.stressLevel * 0.1),
            executionClarity = 0.90,
            directive = "Operating offline. Log goals and thoughts; auto-sync on laptop connection."
        )
    }
}

