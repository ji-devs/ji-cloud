use itertools::Itertools;
use js_sys::Reflect;
use shared::{
    domain::module::{
        body::{
            Body, BodyExt, ModeExt, StepExt,
            _groups::cards::{CardContent, CardPair},
        },
        ModuleBody,
    },
    media::PngImageFile,
};
use utils::{path::image_lib_url, unwrap::UnwrapJiExt};
use wasm_bindgen::JsValue;

const EL_NAME_LIST: &str = "module-card-print-list";
const EL_NAME_SINGLE: &str = "module-card-print-single";
const EL_NAME_DOUBLE: &str = "module-card-print-double";

pub fn print<RawData, Mode, Step>(raw_data: &RawData) -> anyhow::Result<()>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    let html = get_html_to_print(raw_data.as_body())?;

    let custom_elements_script = web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .query_selector("script[src$='/custom-elements.js']")
        .unwrap_ji()
        .unwrap_ji();

    let custom_elements_src = Reflect::get(&custom_elements_script, &JsValue::from_str("src"))
        .unwrap_ji()
        .as_string()
        .unwrap_ji();

    let scripts = vec![custom_elements_src];

    super::print(html, scripts);

    Ok(())
}

fn get_html_to_print(body: Body) -> anyhow::Result<String> {
    let children = match body {
        ModuleBody::MemoryGame(_) | ModuleBody::Matching(_) | ModuleBody::CardQuiz(_) => {
            let pairs = get_card_pairs(body)?;
            cards_to_elements_singles(pairs)
        }
        ModuleBody::Flashcards(_) => {
            let pairs = get_card_pairs(body)?;
            cards_to_elements_doubles(pairs)
        }
        _ => {
            return Err(anyhow::anyhow!("Not a card game"));
        }
    };
    Ok(format!("<{EL_NAME_LIST}>{children}</{EL_NAME_LIST}>"))
}

fn get_card_pairs(body: Body) -> anyhow::Result<Vec<CardPair>> {
    match body {
        ModuleBody::MemoryGame(memory) => Ok(memory.content.unwrap_or_default().base.pairs),
        ModuleBody::Flashcards(flashcards) => Ok(flashcards.content.unwrap_or_default().base.pairs),
        ModuleBody::Matching(matching) => Ok(matching.content.unwrap_or_default().base.pairs),
        ModuleBody::CardQuiz(card_quiz) => Ok(card_quiz.content.unwrap_or_default().base.pairs),
        _ => Err(anyhow::anyhow!("Not a card game")),
    }
}

fn cards_to_elements_singles(cards: Vec<CardPair>) -> String {
    cards
        .into_iter()
        .map(|card| [card.0.card_content, card.1.card_content])
        .flatten()
        .map(|card_content| {
            format!(
                "<{EL_NAME_SINGLE} {attributes}></{EL_NAME_SINGLE}>",
                attributes = get_attributes_for_card_elements(&card_content, "")
            )
        })
        .collect_vec()
        .join("")
}

fn cards_to_elements_doubles(cards: Vec<CardPair>) -> String {
    cards
        .iter()
        .map(|card| {
            format!(
                "<{EL_NAME_DOUBLE} {attributes_0} {attributes_1}></{EL_NAME_DOUBLE}>",
                attributes_0 = get_attributes_for_card_elements(&card.0.card_content, "A"),
                attributes_1 = get_attributes_for_card_elements(&card.1.card_content, "B")
            )
        })
        .collect_vec()
        .join("")
}

fn get_attributes_for_card_elements(card: &CardContent, attr_postfix: &str) -> String {
    match &card {
        CardContent::Text(text) => {
            format!("card{attr_postfix}='{text}' kind{attr_postfix}='text'")
        }
        CardContent::Image(image) => {
            let url = image
                .as_ref()
                .map(|image| image_lib_url(image.lib, PngImageFile::Resized, image.id))
                .unwrap_or_default();
            format!("card{attr_postfix}='{url}' kind{attr_postfix}='image'")
        }
    }
}
