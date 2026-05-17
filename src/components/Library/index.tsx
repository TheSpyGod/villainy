import { Game } from '../../types';

interface Props {
    games: Game[];
    loading: boolean;
}

export function Library({ games, loading }: Props) {
    if (loading) {
        return <p>Loading library...</p>;
    }

    if (games.length === 0) {
        return <p>No games found. Authenticate a store to see your library.</p>;
    }

    return (
        <div>
            <h2>Library ({games.length})</h2>
            <ul>
                {games.map(g => (
                    <li key={g.id}>
                        [{g.store}] {g.title}
                        {g.installed ? ' — installed' : ''}
                    </li>
                ))}
            </ul>
        </div>
    );
}
