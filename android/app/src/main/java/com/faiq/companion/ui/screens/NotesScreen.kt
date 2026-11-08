package com.faiq.companion.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
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

@Composable
fun NotesScreen(repo: SyncRepository) {
    var noteInput by remember { mutableStateOf("") }
    val localNotes = remember { mutableStateListOf<String>() }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(ObsidianBlack)
            .padding(16.dp)
    ) {
        Text("TACTICAL LOG & REFLECTIONS", fontSize = 18.sp, fontWeight = FontWeight.Bold, color = TextPrimary)
        Text("Thoughts logged here auto-sync and grant XP on reconnect", fontSize = 12.sp, color = TextSecondary)

        Spacer(modifier = Modifier.height(16.dp))

        // Note Input
        TextField(
            value = noteInput,
            onValueChange = { noteInput = it },
            placeholder = { Text("Log observation, friction, or completed task...", color = TextSecondary) },
            colors = TextFieldDefaults.colors(
                focusedContainerColor = CardDark,
                unfocusedContainerColor = CardDark,
                focusedTextColor = TextPrimary,
                unfocusedTextColor = TextPrimary,
                cursorColor = AccentGold,
                focusedIndicatorColor = Color.Transparent,
                unfocusedIndicatorColor = Color.Transparent
            ),
            shape = RoundedCornerShape(12.dp),
            modifier = Modifier
                .fillMaxWidth()
                .height(100.dp)
        )

        Spacer(modifier = Modifier.height(8.dp))

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.End
        ) {
            Button(
                onClick = {
                    val note = noteInput.trim()
                    if (note.isNotEmpty()) {
                        repo.queueNote(note)
                        localNotes.add(0, note)
                        noteInput = ""
                    }
                },
                colors = ButtonDefaults.buttonColors(containerColor = AccentGold),
                shape = RoundedCornerShape(8.dp)
            ) {
                Text("Log Note", color = Color.Black, fontWeight = FontWeight.Bold)
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        Text("QUEUED FOR LAPTOP SYNC (${repo.getQueuedNotesCount()})", fontSize = 13.sp, fontWeight = FontWeight.Bold, color = AccentCyan)
        Spacer(modifier = Modifier.height(8.dp))

        LazyColumn(
            modifier = Modifier.weight(1f),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            items(localNotes) { note ->
                Card(
                    colors = CardDefaults.cardColors(containerColor = CardDark),
                    shape = RoundedCornerShape(12.dp),
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(12.dp),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text(note, fontSize = 13.sp, color = TextPrimary, modifier = Modifier.weight(1f))
                        Text("Queued", fontSize = 11.sp, color = AccentGold, fontWeight = FontWeight.SemiBold)
                    }
                }
            }
        }
    }
}

