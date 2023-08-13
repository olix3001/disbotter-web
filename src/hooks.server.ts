import { sequence } from '@sveltejs/kit/hooks';
import { handleErrorWithSentry, sentryHandle } from '@sentry/sveltekit';
import * as Sentry from '@sentry/sveltekit';
import { ProfilingIntegration } from '@sentry/profiling-node';

Sentry.init({
	dsn: 'https://670248e6c2cbf1cff814e53b51c6336c@o4505693048012800.ingest.sentry.io/4505693088251904',
	tracesSampleRate: 1.0,
	profilesSampleRate: 1.0,
	integrations: [new ProfilingIntegration()]
});

// If you have custom handlers, make sure to place them after `sentryHandle()` in the `sequence` function.
export const handle = sequence(sentryHandle());

// If you have a custom error handler, pass it to `handleErrorWithSentry`
export const handleError = handleErrorWithSentry();
