import TopItemShelf from "./TopItemShelf";
import MainContent from "./MainContent";
import "./css/App.css";

function App() {
  return (
    <div className="main-container">
      {" "}
      <TopItemShelf />
      <MainContent />
      <footer>Phone number, Email, Location, Author</footer>
    </div>
  );
}

export default App;
