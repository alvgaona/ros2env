// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://alvgaona.github.io',
	base: '/ros2env',
	integrations: [
		starlight({
			title: 'ros2env',
			description: 'ROS 2 distribution manager for pixi global installations',
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/alvgaona/ros2env',
				},
			],
			editLink: {
				baseUrl: 'https://github.com/alvgaona/ros2env/edit/main/docs/',
			},
			sidebar: [
				{
					label: 'Documentation',
					items: [
						{ label: 'Getting Started', link: '/' },
						{
							label: 'Commands',
							collapsed: false,
							items: [
								{ label: 'Overview', link: '/commands/' },
								{ label: 'Core Commands', link: '/commands/core/' },
								{ label: 'Distribution Management', link: '/commands/distribution/' },
								{ label: 'Utility Commands', link: '/commands/management/' },
							],
						},
					],
				},
				{
					label: 'Contributing',
					link: '/contributing/',
				},
			],
			customCss: [
				'./src/styles/custom.css',
			],
		}),
	],
});
