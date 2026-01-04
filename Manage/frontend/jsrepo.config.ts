import { defineConfig } from 'jsrepo';

export default defineConfig({
    // configure where stuff comes from here
    registries: ['github/DavidHDev/vue-bits'],
    // configure were stuff goes here
    paths: {
        'vue-bits': 'src/components/vue-bits',
    },
});