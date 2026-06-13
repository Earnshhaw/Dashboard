function renderCalendar() {
  const calEl = document.getElementById("calendar");
  if (!calEl) return;

  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth();

  const monthNames = [
    "Január",
    "Február",
    "Marec",
    "Apríl",
    "Máj",
    "Jún",
    "Júl",
    "August",
    "September",
    "Október",
    "November",
    "December",
  ];

  const dayNames = ["Po", "Ut", "St", "Št", "Pi", "So", "Ne"];

  const firstDay = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();

  const offset = firstDay === 0 ? 6 : firstDay - 1;

  let html = `
        <div class="cal-header">${monthNames[month]} ${year}</div>
        <div class="cal-grid">
    `;

  s;
  dayNames.forEach((d) => {
    html += `<div class="cal-dayname">${d}</div>`;
  });

  for (let i = 0; i < offset; i++) {
    html += `<div></div>`;
  }

  for (let d = 1; d <= daysInMonth; d++) {
    const isToday =
      d === now.getDate() &&
      month === now.getMonth() &&
      year === now.getFullYear();

    html += `
            <div class="cal-cell ${isToday ? "cal-today" : ""}">
                ${d}
            </div>
        `;
  }

  html += `</div>`;
  calEl.innerHTML = html;
}

renderCalendar();

function scheduleCalendarRefresh() {
  const now = new Date();
  const msUntilMidnight =
    new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1, 0, 0, 1) -
    now;

  setTimeout(() => {
    renderCalendar();
    scheduleCalendarRefresh();
  }, msUntilMidnight);
}

scheduleCalendarRefresh();
