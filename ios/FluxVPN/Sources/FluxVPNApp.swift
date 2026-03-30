//
//  FluxVPNApp.swift
//  FluxVPN
//
//  Copyright (C) 2026 Flux VPN — Created by Kenji
//  License: GPL-3.0-or-later
//

import SwiftUI

@main
struct FluxVPNApp: App {
    @StateObject private var vpnManager = VPNManager()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(vpnManager)
        }
    }
}

class VPNManager: ObservableObject {
    @Published var isConnected = false
    @Published var isConnecting = false
    @Published var currentServer: String?

    func connect(to server: String = "auto") {
        isConnecting = true
        // TODO: Establish WireGuard tunnel via Network Extension
    }

    func disconnect() {
        isConnected = false
        currentServer = nil
    }
}
