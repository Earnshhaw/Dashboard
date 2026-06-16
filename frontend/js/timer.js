let SCHEDULE = [];

async function loadSchedule() {
  try {
    const res = await fetch("/api/schedule");
    const data = await res.json();

    SCHEDULE = data.schedule;
    console.log("Loaded schedule:", SCHEDULE);

    updateTimeAndSchedule();
    setInterval(updateTimeAndSchedule, 1000);
  } catch (err) {
    console.error("Failed to load schedule:", err);
    document.getElementById("time").innerHTML = `
            <h1 style="color: var(--accent);">Chyba pri načítaní rozvrhu</h1>
        `;
  }
}

loadSchedule();

function toMinutes(t) {
  const [h, m] = t.split(":").map(Number);
  return h * 60 + m;
}

function nowMinutes(now) {
  return now.getHours() * 60 + now.getMinutes();
}

function formatTime(h, m, s) {
  return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s
    .toString()
    .padStart(2, "0")}`;
}

function formatCountdown(totalSeconds) {
  const mm = Math.floor(totalSeconds / 60);
  const ss = totalSeconds % 60;
  return `${mm}m ${ss.toString().padStart(2, "0")}s`;
}

function slovakLessonName(name) {
  const map = {
    "0th Lesson": "nultá hodina",
    "1st Lesson": "1. hodina",
    "2nd Lesson": "2. hodina",
    "3rd Lesson": "3. hodina",
    "4th Lesson": "4. hodina",
    "5th Lesson": "5. hodina",
    "6th Lesson": "6. hodina",
    "7th Lesson": "7. hodina",
    "8th Lesson": "8. hodina",
  };
  return map[name] || name;
}

const TEST_MODE = false;
const TEST_HOUR = 14;
const TEST_MINUTE = 24;

function updateTimeAndSchedule() {
  if (!SCHEDULE.length) return;

  const timeEl = document.getElementById("time");

  const now = new Date();
  if (TEST_MODE) {
    now.setHours(TEST_HOUR);
    now.setMinutes(TEST_MINUTE);
    now.setSeconds(now.getSeconds() + 1);
  }

  const h = now.getHours();
  const m = now.getMinutes();
  const s = now.getSeconds();
  const currentTimeStr = formatTime(h, m, s);

  const current = nowMinutes(now);

  let currentBlock = null;
  let nextBlock = null;

  // Find current and next block
  for (let i = 0; i < SCHEDULE.length; i++) {
    const block = SCHEDULE[i];
    const start = toMinutes(block.start);
    const end = toMinutes(block.end);

    if (current >= start && current < end) {
      currentBlock = block;
      nextBlock = SCHEDULE[i + 1] || null;
      break;
    }

    if (current < start) {
      currentBlock = null;
      nextBlock = block;
      break;
    }
  }

  if (!currentBlock && !nextBlock) {
    timeEl.innerHTML = `
            <div class="time-clock">${currentTimeStr}</div>
            <div class="time-countdown">Koniec vyučovania</div>
            <div class="time-next"></div>
        `;
    return;
  }

  let remaining;
  if (currentBlock) {
    const end = toMinutes(currentBlock.end);
    remaining = end * 60 - (current * 60 + s);
  } else {
    const start = toMinutes(nextBlock.start);
    remaining = start * 60 - (current * 60 + s);
  }

  timeEl.innerHTML = `
        <div class="time-clock">${currentTimeStr}</div>
        <div class="time-countdown">Zvoní o ${formatCountdown(remaining)}</div>
        <div class="time-next">${nextBlock ? `Nasleduje: ${nextBlock.type === "break" ? "prestávka" : slovakLessonName(nextBlock.name)}` : "Koniec vyučovania"}</div>
    `;
}
