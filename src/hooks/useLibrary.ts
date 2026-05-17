import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Game } from '../types';

export function useLibrary() {
    const [games, setGames] = useState<Game[]>([]);
    const [loading, setLoading] = useState(false);

    function refresh() {
        setLoading(true);
        invoke<Game[]>('get_library')
            .then(setGames)
            .catch(err => console.error('[get_library]', err))
            .finally(() => setLoading(false));
    }

    useEffect(() => {
        refresh();
    }, []);

    return { games, loading, refresh };
}
