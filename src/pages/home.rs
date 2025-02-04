// use crate::components::counter_btn::Button;
use crate::components::navigation::nav::Nav;
use leptos::prelude::*;
use crate::components::posts::create_post::CreatePost;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Nav/>
            <main class="min-h-screen bg-white dark:bg-gray-900 dark:text-white text-gray-900 p-4">
            <section class="text-center py-10">
                <h1 class="text-4xl font-bold text-purple-600 dark:text-purple-400">Sociate Circle</h1>
                <p class="mt-4 text-lg text-gray-700 dark:text-gray-300">
                    Entertainment VS News: Share posts to entertain, or provide news links.
                </p>
                <p class="mt-2 text-gray-600 dark:text-gray-400">
                    Earn tokens monthly based on your quality content validated by stakers through score Schelling point community consensus.
                </p>
                <p class="mt-2 text-gray-600 dark:text-gray-400">
                    Improve content through AI for better engagement and quality.
                </p>
            </section>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 px-4">
                <div class="bg-purple-300 dark:bg-purple-700 p-6 rounded-2xl shadow-lg dark:hover:shadow-purple-500 hover:shadow-purple-300 transition-shadow">
                    <h2 class="text-2xl font-semibold">Entertainment</h2>
                    <p class="mt-2 text-gray-800 dark:text-gray-200">Share posts that entertain and engage your audience.</p>
                    <div class="mt-4 bg-purple-500 hover:bg-purple-400 text-white py-2 px-4 rounded-xl">
                        Share Post
                    </div>
                </div>

                <div class="bg-blue-300 dark:bg-blue-700 p-6 rounded-2xl shadow-lg dark:hover:shadow-blue-500 hover:shadow-blue-300 transition-shadow">
                    <h2 class="text-2xl font-semibold">News</h2>
                    <p class="mt-2 text-gray-800 dark:text-gray-200">Provide credible news links to inform the community.</p>
                    <div class="mt-4 bg-blue-500 hover:bg-blue-400 text-white py-2 px-4 rounded-xl">
                        Share News
                    </div>
                </div>

                <div class="bg-green-300 dark:bg-green-700 p-6 rounded-2xl shadow-lg dark:hover:shadow-green-500 hover:shadow-green-300 transition-shadow">
                    <h2 class="text-2xl font-semibold">AI Enhancement</h2>
                    <p class="mt-2 text-gray-800 dark:text-gray-200">Leverage AI tools to improve content quality and engagement.</p>
                    <div class="mt-4 bg-green-500 hover:bg-green-400 text-white py-2 px-4 rounded-xl">
                        Enhance with AI
                    </div>
                </div>
            </div>

            <CreatePost />

            <footer class="text-center mt-10 text-gray-600 dark:text-gray-500">
                "Earn tokens monthly for quality content through community consensus."
            </footer>
        </main>
        </ErrorBoundary>
    }
}
