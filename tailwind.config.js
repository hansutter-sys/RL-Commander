/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        commander: {
          bg: "#0f172a",
          panel: "#1e293b",
          paneActive: "#1e293b",
          paneInactive: "#0f172a",
          border: "#334155",
          accent: "#3b82f6",
          selection: "#1e40af",
          cursor: "#2563eb",
          text: "#f8fafc",
          muted: "#94a3b8",
          dir: "#38bdf8",
          exec: "#4ade80",
          sftp: "#a855f7"
        }
      }
    }
  },
  plugins: []
};
