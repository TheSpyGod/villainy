import { useState } from 'react';
import { SessionStatus, Store } from '../../types';

interface Props {
    sessions: SessionStatus[];
    validating: boolean;
    onAuthenticate: (store: Store) => Promise<void>;
    onLogout: (store: Store) => Promise<void>;
}

export function StoreConnections({ sessions, validating, onAuthenticate, onLogout }: Props) {
    const [busy, setBusy] = useState<Store | null>(null);

    function handleAuth(store: Store) {
        setBusy(store);
        onAuthenticate(store).finally(() => setBusy(null));
    }

    function handleLogout(store: Store) {
        setBusy(store);
        onLogout(store).finally(() => setBusy(null));
    }

    if (validating) {
        return <p>Checking store connections...</p>;
    }

    if (sessions.length === 0) {
        return <p>No supported stores found. Install Legendary, GOGdl, Nile, or Lutris.</p>;
    }

    return (
        <div>
            <h2>Stores</h2>
            {sessions.map(s => (
                <div key={s.store}>
                    <strong>{s.store}</strong>
                    {s.authenticated
                        ? <> — {s.username ?? 'connected'} <button onClick={() => handleLogout(s.store)} disabled={busy === s.store}>Logout</button></>
                        : <> — not connected <button onClick={() => handleAuth(s.store)} disabled={busy === s.store}>{busy === s.store ? 'Waiting for browser...' : 'Authenticate'}</button></>
                    }
                </div>
            ))}
        </div>
    );
}
