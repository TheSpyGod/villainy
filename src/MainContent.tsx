import "./css/Main.css";

export default function MainContent() {
  return (
    <main>
      <section className="main-image-preview">
        <img alt="IMAGE" src="/images/computers.jpg"></img>
        <button> Download </button>
      </section>
      <section className="main-description">
        Description of the project goes here
      </section>
      <section>More info about the project goes here</section>
      <section>The current road map goes here</section>
    </main>
  );
}
