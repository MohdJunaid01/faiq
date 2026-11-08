package com.faiq.companion.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.faiq.companion.data.SyncRepository
import com.faiq.companion.ui.theme.*
import kotlinx.coroutines.launch

@Composable
fun SettingsScreen(repo: SyncRepository) {
    val coroutineScope = rememberCoroutineScope()
    var hostInput by remember { mutableStateOf(repo.laptopHost) }
    var syncMessage by remember { mutableStateOf(repo.lastSyncStatus) }
    var isSyncing by remember { mutableStateOf(false) }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(ObsidianBlack)
            .padding(16.dp)
    ) {
        Text("SETTINGS & SYNCHRONIZATION", fontSize = 18.sp, fontWeight = FontWeight.Bold, color = TextPrimary)
        Text("Configure local Wi-Fi connection to your laptop daemon", fontSize = 12.sp, color = TextSecondary)

        Spacer(modifier = Modifier.height(20.dp))

        Card(
            colors = CardDefaults.cardColors(containerColor = CardDark),
            shape = RoundedCornerShape(12.dp),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text("Laptop Daemon Address", fontSize = 14.sp, fontWeight = FontWeight.SemiBold, color = AccentCyan)
                Text("IP and port of your laptop (e.g. 192.168.1.15:4242 or 10.0.2.2:4242)", fontSize = 11.sp, color = TextSecondary)

                Spacer(modifier = Modifier.height(8.dp))

                TextField(
                    value = hostInput,
                    onValueChange = {
                        hostInput = it
                        repo.laptopHost = it
                    },
                    colors = TextFieldDefaults.colors(
                        focusedContainerColor = SurfaceDark,
                        unfocusedContainerColor = SurfaceDark,
                        focusedTextColor = TextPrimary,
                        unfocusedTextColor = TextPrimary,
                        cursorColor = AccentCyan,
                        focusedIndicatorColor = Color.Transparent,
                        unfocusedIndicatorColor = Color.Transparent
                    ),
                    shape = RoundedCornerShape(8.dp),
                    modifier = Modifier.fillMaxWidth()
                )

                Spacer(modifier = Modifier.height(16.dp))

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        if (isSyncing) "Syncing..." else syncMessage,
                        fontSize = 12.sp,
                        color = if (syncMessage.contains("Synced")) AccentCyan else AccentGold,
                        modifier = Modifier.weight(1f)
                    )

                    Button(
                        onClick = {
                            isSyncing = true
                            coroutineScope.launch {
                                repo.syncWithLaptop()
                                syncMessage = repo.lastSyncStatus
                                isSyncing = false
                            }
                        },
                        colors = ButtonDefaults.buttonColors(containerColor = AccentCyan),
                        shape = RoundedCornerShape(8.dp),
                        enabled = !isSyncing
                    ) {
                        Text("Test & Sync", color = Color.Black, fontWeight = FontWeight.Bold)
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(20.dp))

        // System Architecture Info
        Card(
            colors = CardDefaults.cardColors(containerColor = CardDark),
            shape = RoundedCornerShape(12.dp),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text("HOW SYNCHRONIZATION WORKS", fontSize = 13.sp, fontWeight = FontWeight.Bold, color = AccentGold)
                Spacer(modifier = Modifier.height(6.dp))
                Text("1. Every time you open this app, it checks for your laptop on the local Wi-Fi network.", fontSize = 12.sp, color = TextSecondary)
                Text("2. Any offline messages, tasks, or reflections logged while away are automatically pushed to your laptop.", fontSize = 12.sp, color = TextSecondary)
                Text("3. Faiq processes the queue in SQLite, calculates consolidated XP, and pushes updated levels and distilled lessons back to this device.", fontSize = 12.sp, color = TextSecondary)
            }
        }
    }
}

