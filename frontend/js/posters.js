let posters = [];
let posterIndex = 0;
const POSTER_INTERVAL = 1000 * 60 * 2;

async function loadPosters() {
  try {
    const res = await fetch("/api/posters");
    const data = await res.json();

    if (!Array.isArray(data) || data.length === 0) {
      console.warn("No posters available");
      return;
    }

    posters = data;

    if (posterIndex >= posters.length) {
      posterIndex = 0;
    }
  } catch (err) {
    console.error("Failed to load posters:", err);
  }
}

function showPoster() {
  if (!posters.length) return;

  const img = document.getElementById("poster-img");
  img.src = posters[posterIndex].path;

  posterIndex = (posterIndex + 1) % posters.length;
}

async function cyclePosters() {
  await loadPosters();
  showPoster();

  setTimeout(cyclePosters, POSTER_INTERVAL);
}

cyclePosters();
