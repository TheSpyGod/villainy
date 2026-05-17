import { useSession } from './hooks/useSession';
import { useLibrary } from './hooks/useLibrary';
import { StoreConnections } from './components/StoreConnections';
import { Library } from './components/Library';
import { Store } from './types';

function App() {
    const { sessions, validating, authenticate, logout } = useSession();
    const { games, loading, refresh } = useLibrary();

    function handleAuthenticate(store: Store): Promise<void> {
        return authenticate(store).then(refresh);
    }

    function handleLogout(store: Store): Promise<void> {
        return logout(store).then(refresh);
    }

    return (
        <div className="app">
            <aside className="sidebar">
                <StoreConnections
                    sessions={sessions}
                    validating={validating}
                    onAuthenticate={handleAuthenticate}
                    onLogout={handleLogout}
                />
            </aside>
            <main className="content">
                <Library games={games} loading={loading} />
            </main>
        </div>
    );
}

export default App;
