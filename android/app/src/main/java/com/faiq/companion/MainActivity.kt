package com.faiq.companion

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Chat
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.EditNote
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import com.faiq.companion.data.SyncRepository
import com.faiq.companion.ui.screens.*
import com.faiq.companion.ui.theme.AccentCyan
import com.faiq.companion.ui.theme.FaiqTheme
import com.faiq.companion.ui.theme.ObsidianBlack
import com.faiq.companion.ui.theme.SurfaceDark
import kotlinx.coroutines.launch

enum class Screen(val title: String, val icon: ImageVector) {
    Status("Status", Icons.Default.CheckCircle),
    Chat("Chat", Icons.AutoMirrored.Filled.Chat),
    Notes("Notes", Icons.Default.EditNote),
    Settings("Settings", Icons.Default.Settings)
}

class MainActivity : ComponentActivity() {
    private lateinit var repo: SyncRepository

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        repo = SyncRepository(applicationContext)

        setContent {
            FaiqTheme {
                val coroutineScope = rememberCoroutineScope()
                var currentScreen by remember { mutableStateOf(Screen.Status) }

                // Auto-sync every time the app is opened
                LaunchedEffect(Unit) {
                    repo.syncWithLaptop()
                }

                Scaffold(
                    modifier = Modifier.fillMaxSize(),
                    containerColor = ObsidianBlack,
                    bottomBar = {
                        NavigationBar(
                            containerColor = SurfaceDark
                        ) {
                            Screen.values().forEach { screen ->
                                NavigationBarItem(
                                    selected = currentScreen == screen,
                                    onClick = { currentScreen = screen },
                                    icon = { Icon(screen.icon, contentDescription = screen.title) },
                                    label = { Text(screen.title) },
                                    colors = NavigationBarItemDefaults.colors(
                                        selectedIconColor = ObsidianBlack,
                                        selectedTextColor = AccentCyan,
                                        indicatorColor = AccentCyan,
                                        unselectedIconColor = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f),
                                        unselectedTextColor = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f)
                                    )
                                )
                            }
                        }
                    }
                ) { innerPadding ->
                    Surface(
                        modifier = Modifier
                            .fillMaxSize()
                            .padding(innerPadding),
                        color = ObsidianBlack
                    ) {
                        when (currentScreen) {
                            Screen.Status -> StatusScreen(repo = repo, onSyncClick = {
                                coroutineScope.launch { repo.syncWithLaptop() }
                            })
                            Screen.Chat -> ChatScreen(repo = repo)
                            Screen.Notes -> NotesScreen(repo = repo)
                            Screen.Settings -> SettingsScreen(repo = repo)
                        }
                    }
                }
            }
        }
    }
}

