use leptos::prelude::*;
use leptos_meta::{Style, Title, provide_meta_context};

#[component]
pub fn MyHead() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Rusted NBA: A Rust Based Nights Black Agents' Character Creator!"/>
        <Style>
        "
        @import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@700&display=swap');

        /* BACKGROUND */
        body {
            background-color: #121212 !important;
            color: #E0E0E0;
        }

        /* CHARACTER SHEET */
        #character-sheet {
            background-color: #1E1E1E !important;
            padding: 20px;
            margin: 20px;
            border-radius: 12px;
            box-shadow: 0 0 20px rgba(0, 255, 150, 0.2);
        }


        /* AGENT INFO SECTION */
        #agent-info {
            display: flex;
            flex-wrap: wrap;
            align-items: center;
            justify-content: flex-start;
            gap: 20px;
            margin-bottom: 20px;
        }

        #agent-info label {
            color: red !important;
            font-family: 'Orbitron', sans-serif !important;
            font-size: 1.2em !important;
            font-weight: bold !important;
        }

        #general-skills label {
            display: inline-block;
            width: 100px;
            text-align: left;
        }

        #general-skills > div {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        #general-skills .bubbles {
            display: flex;
            gap: 5px;
        }

        #general-skills input[type='checkbox'] {
            margin-left: auto;
        }

        #investigative-skills {
            display: flex;
            justify-content: space-between;
            gap: 5%; /* You can still use a gap for even spacing if needed */
        }
        #investigative-skills > div {
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            white-space: nowrap; /* Prevents text from breaking onto new lines */
        }

        .bubbles {
            margin-bottom: 10px;
        }

        #investigative-skills header {
            margin-bottom: 10px;
            font-size: 2em;
            font-weight: bold;
        }


        #skills {
            display: flex;
            flex-direction: row;
            gap: 20px;
        }

        "
        </Style>
    }
}
