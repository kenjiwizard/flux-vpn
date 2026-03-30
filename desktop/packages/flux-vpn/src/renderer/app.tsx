/**
 * Flux VPN — Desktop GUI (Renderer)
 *
 * Copyright (C) 2026 Flux VPN — Created by Kenji
 * License: GPL-3.0-or-later
 */

import React, { useState } from 'react';

type ConnectionStatus = 'disconnected' | 'connecting' | 'connected';

export default function App() {
  const [status, setStatus] = useState<ConnectionStatus>('disconnected');

  const handleToggle = () => {
    if (status === 'disconnected') {
      setStatus('connecting');
      setTimeout(() => setStatus('connected'), 2000);
    } else {
      setStatus('disconnected');
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>Flux VPN</h1>
      </header>
      <main className="app-main">
        <div className={`status-indicator ${status}`} />
        <p className="status-text">
          {status === 'disconnected' && 'Not connected'}
          {status === 'connecting' && 'Connecting...'}
          {status === 'connected' && 'Connected & Secure'}
        </p>
        <button className="connect-btn" onClick={handleToggle}>
          {status === 'disconnected' ? 'Connect' : 'Disconnect'}
        </button>
      </main>
    </div>
  );
}
