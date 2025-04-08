use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL, *};

const GRID_WIDTH: usize = 10;
const GRID_EPSILON: f32 = 0.05; // 오차 허용

#[derive(Clone)]
pub struct Block {
    pub x: f32,
    pub y: f32,
}

pub struct GameState {
    current: Block,
    blocks: Vec<Block>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current: Block { x: 0.0, y: 1.0 },
            blocks: vec![],
        }
    }

    fn clear_full_rows(&mut self) {
        let mut rows: std::collections::HashMap<i32, Vec<&Block>> = std::collections::HashMap::new();

        // y좌표를 정수화해서 각 줄에 블록 분류
        for b in &self.blocks {
            let row = (b.y / GRID_EPSILON).round() as i32;
            rows.entry(row).or_default().push(b);
        }

        // 꽉 찬 줄만 추출
        let full_rows: Vec<i32> = rows
            .iter()
            .filter(|(_, bs)| bs.len() >= GRID_WIDTH)
            .map(|(y, _)| *y)
            .collect();

        if full_rows.is_empty() {
            return;
        }

        // 해당 줄 제거
        self.blocks.retain(|b| {
            let row = (b.y / GRID_EPSILON).round() as i32;
            !full_rows.contains(&row)
        });

        // 위의 블록들 아래로 이동
        for b in &mut self.blocks {
            let row = (b.y / GRID_EPSILON).round() as i32;
            let count = full_rows.iter().filter(|r| row > **r).count() as f32;
            b.y -= count * GRID_EPSILON;
        }
    }

    pub fn update(&mut self) {
        self.current.y -= 0.01;

        let hit_bottom = self.current.y <= -0.9;
        let hit_block = self.check_collision();

        if hit_bottom || hit_block {
            self.blocks.push(self.current.clone());
            self.clear_full_rows(); // 🎯 이 줄 추가
            self.current = Block { x: 0.0, y: 1.0 };
        }
    }

    fn check_collision(&self) -> bool {
        for b in &self.blocks {
            let dy = b.y - self.current.y;
            let dx = (b.x - self.current.x).abs();
            if dy < 0.2 && dy > 0.0 && dx < 0.2 {
                return true;
            }
        }
        false
    }

    pub fn move_left(&mut self) {
        self.current.x -= 0.05;
    }

    pub fn move_right(&mut self) {
        self.current.x += 0.05;
    }

    pub fn move_down(&mut self) {
        self.current.y -= 0.05;
    }

    pub fn current(&self) -> Block {
        self.current.clone()
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}

#[wasm_bindgen]
pub struct Game {
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
        }
    }

    pub fn update(&mut self) {
        self.state.update();
    }

    pub fn move_left(&mut self) {
        self.state.move_left();
    }

    pub fn move_right(&mut self) {
        self.state.move_right();
    }

    pub fn move_down(&mut self) {
        self.state.move_down();
    }

    pub fn current_x(&self) -> f32 {
        self.state.current().x
    }

    pub fn current_y(&self) -> f32 {
        self.state.current().y
    }

    pub fn blocks_x(&self) -> Vec<f32> {
        self.state.blocks().iter().map(|b| b.x).collect()
    }

    pub fn blocks_y(&self) -> Vec<f32> {
        self.state.blocks().iter().map(|b| b.y).collect()
    }
}

// JS에서 호출 할 수 있도록 설정
#[wasm_bindgen]
pub fn init_webgl() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window.document().ok_or("Should have a document on window")?;
    let canvas = document
        .get_element_by_id("gameCanvas")
        .ok_or("Canvas not found")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .ok_or("Failed to get WebGL context")?
        .dyn_into::<WebGlRenderingContext>()?;

    // 캔버스 배경색 설정
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}

#[wasm_bindgen]
pub fn draw_block(x: f32, y: f32) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;
    let canvas = document
        .get_element_by_id("gameCanvas")
        .ok_or("no canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .ok_or("no webgl")?
        .dyn_into::<WebGlRenderingContext>()?;

    gl.clear_color(0.1, 0.1 + x, 0.4 + y, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}

#[wasm_bindgen]
pub fn draw_square() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas = document
        .get_element_by_id("gameCanvas")
        .ok_or("no canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .ok_or("no webgl")?
        .dyn_into()?;

    // 셰이더 정의
    let vert_code = r#"
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let frag_code = r#"
        void main() {
            gl_FragColor = vec4(0.0, 1.0, 0.5, 1.0);
        }
    "#;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_code)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_code)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    // 정점 버퍼 설정 (정사각형 두 삼각형으로 구성)
    let vertices: [f32; 12] = [
        -0.5, -0.5,
         0.5, -0.5,
        -0.5,  0.5,
        -0.5,  0.5,
         0.5, -0.5,
         0.5,  0.5,
    ];

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    // 메모리에 정점 데이터 전달
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    }

    let position = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);

    gl.draw_arrays(GL::TRIANGLES, 0, 6);

    Ok(())

}

// 셰이더 컴파일 함수
fn compile_shader(gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or("Unknown error".into()))
    }
}

// 셰이더 프로그램 링크 함수
fn link_program(gl: &WebGlRenderingContext, vert: &WebGlShader, frag: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or("Unable to create program")?;
    gl.attach_shader(&program, vert);
    gl.attach_shader(&program, frag);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or("Unknown link error".into()))
    }
}

#[wasm_bindgen]
pub fn draw_square_at(x: f32, y: f32) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas = document
        .get_element_by_id("gameCanvas")
        .ok_or("no canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .ok_or("no webgl")?
        .dyn_into()?;

    let vert_code = r#"
        attribute vec2 position;
        uniform vec2 offset;
        void main() {
            gl_Position = vec4(position + offset, 0.0, 1.0);
        }
    "#;

    let frag_code = r#"
        void main() {
            gl_FragColor = vec4(0.2, 0.8, 1.0, 1.0);
        }
    "#;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_code)?;
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_code)?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 12] = [
        -0.1, -0.1,
         0.1, -0.1,
        -0.1,  0.1,
        -0.1,  0.1,
         0.1, -0.1,
         0.1,  0.1,
    ];

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
    }

    let pos_attrib = gl.get_attrib_location(&program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(pos_attrib, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(pos_attrib);

    let offset_loc = gl.get_uniform_location(&program, "offset");
    gl.uniform2f(offset_loc.as_ref(), x, y);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);

    gl.draw_arrays(GL::TRIANGLES, 0, 6);

    Ok(())
}