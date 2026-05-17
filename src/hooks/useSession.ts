import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { SessionStatus, Store } from '../types';

export function useSession() {
    const [sessions, setSessions] = useState<SessionStatus[]>([]);
    const [validating, setValidating] = useState(true);

    useEffect(() => {
        invoke<SessionStatus[]>('validate_sessions')
            .then(setSessions)
            .finally(() => setValidating(false));
    }, []);

    function authenticate(store: Store): Promise<void> {
        return invoke<SessionStatus>('authenticate', { store_name: store })
            .then(updated => {
                setSessions(prev =>
                    prev.map(s => s.store === store ? updated : s)
                );
            });
    }

    function logout(store: Store): Promise<void> {
        return invoke<void>('logout', { store_name: store })
            .then(() => {
                setSessions(prev =>
                    prev.map(s => s.store === store
                        ? { ...s, authenticated: false, username: undefined }
                        : s
                    )
                );
            });
    }

    return { sessions, validating, authenticate, logout };
}
