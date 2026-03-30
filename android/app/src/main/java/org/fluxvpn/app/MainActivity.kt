/**
 * Flux VPN — Android
 *
 * Copyright (C) 2026 Flux VPN — Created by Kenji
 * License: GPL-3.0-or-later
 */

package org.fluxvpn.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            FluxVpnApp()
        }
    }
}
