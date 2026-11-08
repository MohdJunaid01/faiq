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
import kotlinx.coroutines.launch

data class UiChatMessage(
    val sender: String, // "You" or "Faiq"
    val text: String,
    val composure: Double? = null
)

@Composable
fun ChatScreen(repo: SyncRepository) {
    val coroutineScope = rememberCoroutineScope()
    var inputText by remember { mutableStateOf("") }
    val messages = remember {
        mutableStateListOf(
            UiChatMessage(
                sender = "Faiq",
                text = "Operational. State your current objective or evaluation target.",
                composure = 0.99
            )
        )
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(ObsidianBlack)
            .padding(16.dp)
    ) {
        // Chat Header
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text("STRATEGIC SPARRING", fontSize = 18.sp, fontWeight = FontWeight.Bold, color = TextPrimary)
                Text("Unfiltered Alter Ego Dialogue", fontSize = 12.sp, color = TextSecondary)
            }
            if (repo.getQueuedMessagesCount() > 0) {
                Surface(
                    color = AccentGold.copy(alpha = 0.2f),
                    shape = RoundedCornerShape(12.dp)
                ) {
                    Text(
                        "${repo.getQueuedMessagesCount()} queued",
                        modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
                        color = AccentGold,
                        fontSize = 11.sp
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // Message List
        LazyColumn(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth(),
            verticalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            items(messages) { msg ->
                val isUser = msg.sender == "You"
                Column(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalAlignment = if (isUser) Alignment.End else Alignment.Start
                ) {
                    Text(
                        msg.sender,
                        fontSize = 11.sp,
                        color = if (isUser) AccentCyan else AccentGold,
                        fontWeight = FontWeight.Bold
                    )
                    Spacer(modifier = Modifier.height(2.dp))
                    Surface(
                        color = if (isUser) CardDark else SurfaceDark,
                        shape = RoundedCornerShape(12.dp),
                        modifier = Modifier.widthIn(max = 300.dp)
                    ) {
                        Column(modifier = Modifier.padding(12.dp)) {
                            Text(msg.text, fontSize = 14.sp, color = TextPrimary)
                            if (msg.composure != null) {
                                Spacer(modifier = Modifier.height(4.dp))
                                Text(
                                    "Composure: ${String.format("%.1f", msg.composure * 100.0)}%",
                                    fontSize = 10.sp,
                                    color = TextSecondary
                                )
                            }
                        }
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(8.dp))

        // Input Field
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            TextField(
                value = inputText,
                onValueChange = { inputText = it },
                placeholder = { Text("Speak with Faiq...", color = TextSecondary) },
                colors = TextFieldDefaults.colors(
                    focusedContainerColor = CardDark,
                    unfocusedContainerColor = CardDark,
                    focusedTextColor = TextPrimary,
                    unfocusedTextColor = TextPrimary,
                    cursorColor = AccentCyan,
                    focusedIndicatorColor = Color.Transparent,
                    unfocusedIndicatorColor = Color.Transparent
                ),
                shape = RoundedCornerShape(12.dp),
                modifier = Modifier.weight(1f)
            )

            Spacer(modifier = Modifier.width(8.dp))

            Button(
                onClick = {
                    val textToSend = inputText.trim()
                    if (textToSend.isNotEmpty()) {
                        messages.add(UiChatMessage("You", textToSend))
                        inputText = ""
                        coroutineScope.launch {
                            val resp = repo.sendChatMessage(textToSend)
                            messages.add(UiChatMessage("Faiq", resp.reply, resp.composure))
                        }
                    }
                },
                colors = ButtonDefaults.buttonColors(containerColor = AccentCyan),
                shape = RoundedCornerShape(12.dp),
                modifier = Modifier.height(56.dp)
            ) {
                Text("Send", color = Color.Black, fontWeight = FontWeight.Bold)
            }
        }
    }
}

