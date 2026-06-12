export const environment = {
  production: false,
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
    enabled: true,
    level: 'debug',
    levels: {
      debug: true,
      info: true,
      warn: true,
      error: true,
    },
    sources: {
      view: true,
      service: true,
      store: true,
      api: true,
      user: true,
      router: true,
    },
  },
};
