const WEATHER_ICONS = {
  0: "☀️", // clear
  1: "🌤️", // mostly clear
  2: "⛅", // partly cloudy
  3: "☁️", // overcast
  45: "🌫️", // fog
  48: "🌫️",
  51: "🌦️", // drizzle
  53: "🌦️", //
  61: "🌧️", // rain
  71: "🌨️", // snow
  95: "⛈️", // thunderstorm
};

const weatherEl = document.getElementById("weather");
const WEATHER_URL = "/api/weather";

async function loadWeather() {
  try {
    const res = await fetch(WEATHER_URL);
    if (!res.ok) throw new Error("Failed to fetch weather");

    const data = await res.json();

    const icon = WEATHER_ICONS[data.weather_code] || "❓";

    weatherEl.innerHTML = `

            <div class="weather-main">
                <div class="weather-icon">${icon}</div>
                <div class="weather-temp">${Math.round(data.temperature)}°C</div>
            </div>

            <div class="weather-details">
                <div>Zdanlivá teplota: <strong>${Math.round(data.apparent_temperature)}°C</strong></div>
                <div>Vlhkosť: <strong>${data.humidity}%</strong></div>
                <div>Vietor: <strong>${Math.round(data.wind_speed)} km/h</strong></div>
                <div>Zamračenie: <strong>${data.cloud_cover}%</strong></div>
            </div>
        `;
  } catch (err) {
    console.error("Weather error:", err);
    weatherEl.innerHTML = `
      <div class="weather-main">
          <div class="weather-icon"></div>
          <div class="weather-temp">°C</div>
      </div>

      <div class="weather-details">
          <div>Zdanlivá teplota: <strong>°C</strong></div>
          <div>Vlhkosť: <strong>%</strong></div>
          <div>Vietor: <strong> km/h</strong></div>
          <div>Zamračenie: <strong>%</strong></div>
      </div>
        `;
  }
}

loadWeather();
setInterval(loadWeather, 1000 * 60 * 10);
