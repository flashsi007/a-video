/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#ff4c4c',
        background: {
          DEFAULT: '#18181c',
          light: '#23232a',
          card: '#23232a',
        },
      },
      borderRadius: {
        '2xl': '1.25rem',
        '3xl': '1.5rem',
      },
      boxShadow: {
        card: '0 4px 24px 0 rgba(0,0,0,0.25)',
        '2xl': '0 8px 32px 0 rgba(0,0,0,0.35)',
      },
      ringWidth: {
        2: '2px',
      },
      ringColor: {
        primary: '#ff4c4c',
      },
    },
  },
  fontFamily: {
    sans: ['Inter', 'PingFang SC', 'Microsoft YaHei', 'Arial', 'sans-serif']
  },
  boxShadow: {
    'card': '0 4px 24px 0 rgba(0,0,0,0.25)',
    'focus': '0 0 0 2px #2563eb55'
  },
  borderRadius: {
    'xl': '1rem',
    '2xl': '1.5rem'
  }
}

