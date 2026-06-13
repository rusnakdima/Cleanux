export const environment = {
  production: false,
  logging: {
    enabled: true,
    level: 'info',

    levels: {
      debug: false,
      info: true,
      warn: true,
      error: true,
    },

    sources: {
      view: true,
      service: true,
      store: false,
      api: true,
      user: false,
      router: false,
    },
  },
};
