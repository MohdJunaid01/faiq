package com.faiq.companion.model

data class NumericTraits(
    val confidence: Double = 80.0,
    val resilience: Double = 90.0,
    val intelligence: Double = 85.0,
    val calculation: Double = 92.0,
    val courage: Double = 88.0
)

data class InnerState(
    val stressLevel: Double = 0.1,
    val focus: Double = 0.9,
    val urgency: Double = 0.2
)

data class LevelEngine(
    val level: Int = 1,
    val xp: Double = 0.0,
    val totalExperiences: Long = 0
)

data class Identity(
    val name: String = "Faiq",
    val age: Int = 20,
    val appearance: String = "Slicked-back hair, black blazer, calculating smile",
    val livesAsReal: Boolean = true
)

data class FaiqState(
    val identity: Identity = Identity(),
    val traits: NumericTraits = NumericTraits(),
    val innerState: InnerState = InnerState(),
    val levelEngine: LevelEngine = LevelEngine()
)

data class TacticalGoal(
    val id: String,
    val title: String,
    val description: String,
    val difficulty: Double,
    val status: String
)

data class Lesson(
    val id: Long?,
    val principle: String,
    val context: String,
    val relevanceCount: Long
)

data class OfflineMessage(
    val timestampRfc3339: String,
    val userPrompt: String,
    val response: String? = null
)

data class SyncPushRequest(
    val deviceId: String,
    val offlineMessages: List<OfflineMessage>,
    val offlineNotes: List<String>
)

data class SyncPushResponse(
    val acceptedEvents: Int,
    val xpAwarded: Double,
    val state: FaiqState,
    val activeGoals: List<TacticalGoal>,
    val topLessons: List<Lesson>
)

data class ApiChatResponse(
    val reply: String,
    val level: Int,
    val xp: Double,
    val composure: Double,
    val executionClarity: Double,
    val directive: String
)

