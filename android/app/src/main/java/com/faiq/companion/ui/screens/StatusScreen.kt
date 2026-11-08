package com.faiq.companion.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.faiq.companion.data.SyncRepository
import com.faiq.companion.ui.theme.*

@Composable
fun StatusScreen(repo: SyncRepository, onSyncClick: () -> Unit) {
    val state = repo.currentState
    val traits = state.traits
    val level = state.levelEngine.level
    val xp = state.levelEngine.xp
    val xpMax = level * 100.0

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(ObsidianBlack)
            .padding(16.dp)
            .verticalScroll(rememberScrollState())
    ) {
        // Header
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text("FAIQ", fontSize = 24.sp, fontWeight = FontWeight.Bold, color = TextPrimary)
                Text("Virtual Alter Ego • Age 20", fontSize = 12.sp, color = TextSecondary)
            }
            Surface(
                color = AccentCyan.copy(alpha = 0.2f),
                shape = RoundedCornerShape(16.dp)
            ) {
                Text(
                    "LEVEL $level",
                    modifier = Modifier.padding(horizontal = 12.dp, vertical = 6.dp),
                    color = AccentCyan,
                    fontWeight = FontWeight.Bold,
                    fontSize = 14.sp
                )
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        // XP Progress
        Card(
            colors = CardDefaults.cardColors(containerColor = CardDark),
            shape = RoundedCornerShape(12.dp),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text("XP Progression", fontSize = 14.sp, color = TextSecondary)
                    Text("${String.format("%.1f", xp)} / ${String.format("%.1f", xpMax)}", fontSize = 14.sp, color = AccentCyan)
                }
                Spacer(modifier = Modifier.height(8.dp))
                LinearProgressIndicator(
                    progress = { (xp / xpMax).toFloat().coerceIn(0f, 1f) },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(8.dp)
                        .clip(RoundedCornerShape(4.dp)),
                    color = AccentCyan,
                    trackColor = SurfaceDark,
                )
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        // Traits Breakdown
        Text("NUMERIC TRAITS", fontSize = 14.sp, fontWeight = FontWeight.Bold, color = AccentGold)
        Spacer(modifier = Modifier.height(8.dp))

        Card(
            colors = CardDefaults.cardColors(containerColor = CardDark),
            shape = RoundedCornerShape(12.dp),
            modifier = Modifier.fillMaxWidth()
        ) {
            Column(modifier = Modifier.padding(16.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                TraitBar("Confidence", traits.confidence)
                TraitBar("Resilience", traits.resilience)
                TraitBar("Intelligence", traits.intelligence)
                TraitBar("Calculation", traits.calculation)
                TraitBar("Courage", traits.courage)
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        // Distilled Lessons
        Text("TOP DISTILLED LESSONS", fontSize = 14.sp, fontWeight = FontWeight.Bold, color = AccentCyan)
        Spacer(modifier = Modifier.height(8.dp))

        if (repo.topLessons.isEmpty()) {
            Card(
                colors = CardDefaults.cardColors(containerColor = CardDark),
                shape = RoundedCornerShape(12.dp),
                modifier = Modifier.fillMaxWidth()
            ) {
                Text(
                    "No distilled lessons synced yet. Experience will forge them.",
                    modifier = Modifier.padding(16.dp),
                    fontSize = 13.sp,
                    color = TextSecondary
                )
            }
        } else {
            Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                repo.topLessons.take(3).forEach { lesson ->
                    Card(
                        colors = CardDefaults.cardColors(containerColor = CardDark),
                        shape = RoundedCornerShape(12.dp),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Column(modifier = Modifier.padding(12.dp)) {
                            Text("• \"${lesson.principle}\"", fontSize = 13.sp, fontWeight = FontWeight.SemiBold, color = TextPrimary)
                            Text("Context: ${lesson.context}", fontSize = 11.sp, color = TextSecondary)
                        }
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        // Sync Status Footer
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(repo.lastSyncStatus, fontSize = 12.sp, color = TextSecondary)
            Button(
                onClick = onSyncClick,
                colors = ButtonDefaults.buttonColors(containerColor = AccentCyan),
                shape = RoundedCornerShape(8.dp)
            ) {
                Text("Sync Now", color = Color.Black, fontWeight = FontWeight.Bold)
            }
        }
    }
}

@Composable
fun TraitBar(name: String, value: Double) {
    Column {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(name, fontSize = 12.sp, color = TextPrimary)
            Text(String.format("%.1f", value), fontSize = 12.sp, fontWeight = FontWeight.Bold, color = AccentGold)
        }
        Spacer(modifier = Modifier.height(4.dp))
        LinearProgressIndicator(
            progress = { (value / 100.0).toFloat().coerceIn(0f, 1f) },
            modifier = Modifier
                .fillMaxWidth()
                .height(4.dp)
                .clip(RoundedCornerShape(2.dp)),
            color = AccentGold,
            trackColor = SurfaceDark,
        )
    }
}

