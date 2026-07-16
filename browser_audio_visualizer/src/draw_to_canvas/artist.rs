use palette::IntoColor;
use wasm_bindgen::JsCast;
//use web_sys::features::gen_HtmlCanvasElement::HtmlCanvasElement;
//
use web_sys::{
    HtmlCanvasElement, PermissionState, PermissionStatus, WebGl2RenderingContext,
    js_sys::Intl::{DurationFormatPartType::Milliseconds, RelativeTimeFormatUnit::Seconds},
    window,
}; //drawing a circle

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use web_sys::*;

use limelight::{
    DrawMode, Renderer, renderer::Drawable, state::blending::BlendingFactorSrc::DstAlpha,
}; //also for drawing the circle

use anyhow::{Result, anyhow};

pub struct Artist {
    gl: WebGl2RenderingContext,
    //limelight: ,*/
}

impl Artist {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        match try_get_gl(canvas) {
            Ok(gl) => Ok(Self { gl }),
            Err(err) => return Err(err),
        }
    }
}

fn try_get_gl(canvas: HtmlCanvasElement) -> Result<WebGl2RenderingContext, String> {
    let gl = canvas.get_context("webgl2").map_err(|e| {
        jsval_to_err(
            e.as_string(),
            "get_context failed without an error message".to_string(),
        )
    })?;

    if let Some(gl) = gl {
        return Ok(gl.dyn_into::<WebGl2RenderingContext>().map_err(|e| {
            jsval_to_err(
                e.as_string(),
                "Object converting to WebGl2RenderingContext".to_string(),
            )
        })?);
    } else {
        Err("gl object was empty".to_string())
    }
}

fn jsval_to_err(e: Option<String>, error_context: String) -> String {
    match e {
        Some(error) => error,
        None => format!(
            "Something went wrong and there's no error message. context: {}",
            error_context
        ),
    }
}

/*
* future reference
*
* use anyhow::{anyhow, Result};

fn try_get_gl(canvas: HtmlCanvasElement) -> Result<WebGl2RenderingContext> {
    let gl = canvas.get_context("webgl2")
        .map_err(|_| anyhow!("get_context failed"))?
        .ok_or(anyhow!("gl object was empty"))?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| anyhow!("failed to convert to WebGL context"))?;
    Ok(gl)
}

it's ai generated but the idea is something i want to remember and explore next time i do something similar
*/
