const namedayEl = document.getElementById("nameday");
const NAMEDAY_URL = "/api/nameday";

async function loadNameday() {
  try {
    const res = await fetch(NAMEDAY_URL);
    if (!res.ok) throw new Error("Failed to fetch nameday");

    const data = await res.json();

    namedayEl.innerHTML = `

            <div class="nameday-today">Dnes má meniny ${data.nameday}</div>
        `;
  } catch (err) {
    console.error("Nameday error:", err);
    namedayEl.innerHTML = `

            <div class="nameday-today muted">Unavailable</div>
        `;
  }
}

function scheduleMidnightRefresh() {
  const now = new Date();
  const next = new Date();
  next.setHours(24, 0, 0, 0);

  setTimeout(() => {
    loadNameday();
    scheduleMidnightRefresh();
  }, next - now);
}

loadNameday();
scheduleMidnightRefresh();
