export const environment = {
  production: true,
  version: '0.2.1',
  gitRepoName: 'Cleanux',
  githubUser: 'TechCraft-Solutions',
  nameProduct: 'Cleanux',
  yearCreate: 2026,
  companyName: 'TechCraft Solutions',
  authors: [
    {
      name: 'Dmitriy303',
      email: 'rusnakdima03@gmail.com',
      url: 'https://github.com/rusnakdima',
    },
  ],
  logging: {
    enabled: false,
    level: 'error',

    levels: {
      debug: false,
      info: false,
      warn: true,
      error: true,
    },

    sources: {
      view: false,
      service: false,
      store: false,
      api: false,
      user: false,
      router: false,
    },
  },
};
