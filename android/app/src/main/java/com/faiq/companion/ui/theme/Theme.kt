package com.faiq.companion.ui.theme

import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

val ObsidianBlack = Color(0xFF0F1115)
val SurfaceDark = Color(0xFF161A22)
val CardDark = Color(0xFF212631)
val AccentCyan = Color(0xFF38BDF8)
val AccentGold = Color(0xFFFBBF24)
val TextPrimary = Color(0xFFF3F4F6)
val TextSecondary = Color(0xFF9CA3AF)

private val DarkColorScheme = darkColorScheme(
    primary = AccentCyan,
    secondary = AccentGold,
    background = ObsidianBlack,
    surface = SurfaceDark,
    onPrimary = Color.Black,
    onSecondary = Color.Black,
    onBackground = TextPrimary,
    onSurface = TextPrimary
)

@Composable
fun FaiqTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = DarkColorScheme,
        typography = Typography(),
        content = content
    )
}

