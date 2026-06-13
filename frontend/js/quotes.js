async function loadQuote() {
  const quoteEl = document.getElementById("quote");
  if (!quoteEl) return;

  try {
    const res = await fetch("/api/quotes");
    const data = await res.json();

    const list = data.quotes;
    if (!Array.isArray(list) || list.length === 0) {
      quoteEl.innerHTML = `<div class="text">Žiadne citáty k dispozícii</div>`;
      return;
    }

    const q = list[Math.floor(Math.random() * list.length)];

    quoteEl.innerHTML = `
            <div class="text">"${q.text}"</div>
            <div class="author">${q.author}</div>
        `;
  } catch (err) {
    console.error("Failed to load quotes:", err);
    quoteEl.innerHTML = `<div class="text" style="color: var(--accent);">Chyba pri načítaní citátu</div>`;
  }
}

loadQuote();

setInterval(loadQuote, 1000 * 60 * 5);
