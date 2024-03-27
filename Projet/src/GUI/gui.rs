use druid::widget::{Button, Flex, ViewSwitcher, Label, CrossAxisAlignment, MainAxisAlignment};
use druid::{
    commands, AppDelegate, AppLauncher, Command, DelegateCtx, Env, FileDialogOptions, FileSpec,
    Handled, LocalizedString, Target, Widget, WindowDesc, Data, Lens, WidgetExt
};

use std::io::BufReader;

struct Delegate;

#[derive(Clone, Data, Lens)]
struct AppState {
    current_view: u32,
    current_text: String,
}

/* 
struct audio_file_path {
    file_path: String
}
*/

pub fn launch() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("gui").with_placeholder("GUI"));
    let mut data = AppState {
        current_view: 0,
        current_text: "".to_string(),
    };
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(data)
        .expect("launch failed");
}


fn ui_builder() -> impl Widget<AppState> {


    /* 
    let mut start_path = std::env::current_dir().unwrap();
    start_path.push("..");
    start_path.push("assets");
    start_path.push("audios");
    start_path.push("src");
    */
    //let srcfile="";
    //let targetfile="";
    /*
    let audio = FileSpec::new("Audio files", &["mp3","wav"]);
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![audio])
        .name_label("Source")
        .title("Choose the source audio file")
        .button_text("Import");

        let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });
    */

    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.clone(),
        |selector, _data, _env| match selector {
            AppState{current_view:0, current_text} => Box::new(
                Flex::column()
                    .with_flex_child(Label::new("Please select an audio file").center(), 1.0)
                    .with_flex_child(Button::new("Import File").on_click(move |ctx, data: &mut AppState, _| {
                let audio = FileSpec::new("Audio files", &["mp3","wav"]);
                let open_dialog_options = FileDialogOptions::new()
                    .allowed_types(vec![audio])
                    .name_label("Source")
                    .title("Choose the source audio file")
                    .button_text("Import")
                    //.force_starting_directory(start_path.clone())
                    //.accept_command()
                    ;
                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()));
                //*data=1;
            })
            .center(),
            1.0)
            //.lens(AppState::current_view)
            ),
            AppState{current_view:1, current_text}=>Box::new(
                Flex::column()
                .with_flex_child(Label::new("What would you like to do").center(), 1.0)
                .with_flex_child(Flex::row()
                                        .with_flex_child(Button::new("Pitch Track").center().on_click(|_ctx, data: &mut AppState, _| {data.current_view=2;}), 1.0)
                                        .with_flex_child(Button::new("Pitch Shift").center().on_click(|_ctx, data: &mut AppState, _| {data.current_view=3;}), 1.0)
                                        .with_flex_child(Button::new("AutoTune").center(), 1.0)
                                        , 1.0)
                //.lens(AppState::current_view)
            ),
            AppState{current_view:2, current_text}=>Box::new(
                Flex::column()
                .with_flex_child(Label::new("Pitch Tracking").center(), 1.0)
                .with_flex_child(Button::new("Restart").center().on_click(|_ctx, data: &mut AppState, _| {data.current_view=0;}), 1.0)
                //.lens(AppState::current_view)
            ),

            AppState{current_view:3, current_text}=>Box::new(
                Flex::column()
                .with_flex_child(Label::new("Pitch Shifting").center(), 1.0)
                                        .with_flex_child(Button::new("Restart").center().on_click(|_ctx, data: &mut AppState, _| {
                                            data.current_view=0;}), 1.0)
                                        .with_flex_child(Button::new("Play Audio").center().on_click(|_ctx, data:  &mut AppState, _| {
                                                //let current_text = data.current_text.clone();
                                                play_audio((data.current_text).to_string());
                                            }), 1.0)
                //.lens(AppState::current_view)
            ),
            _ => Box::new(Label::new("Unknown").center()),
        });


    Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_child(view_switcher, 1.0)
        
}


fn play_audio(output_path : String){
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(output_path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}




impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.current_text = file_info.path().to_string_lossy().to_string();
            //audio_file_path = &file_info.path().to_string_lossy().to_string();
            data.current_view = 1;
            /* 
            match std::fs::read(file_info.path()) {
                Ok(bytes) => {
                    let content = String::from_utf8_lossy(&bytes);
                    data.current_text = content.to_string();
                    println!({}, data.current_text);
                    data.current_view = 1;
                    // ctx.submit_command(druid::commands::REQUEST_UPDATE);
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            */
            return Handled::Yes;
        }
        Handled::No
    }
}




/* 
fn main(){
    choose_file();
}

fn choose_file() {
    let mut path = std::env::current_dir().unwrap();
    path.push("audios");
    path.push("src");
    let res: Option<Vec<std::path::PathBuf>> = rfd::FileDialog::new()
        .add_filter("audio", &["wav", "mp3"])
        .set_directory(&path)
        .pick_files();

    println!("The user choose: {}", path_formating(res));
}
    

fn path_formating(res: Option<Vec<std::path::PathBuf>>) -> String {
    match res {
        Some(paths) => paths
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join(", "), // or any separator you prefer
        None => String::from("No files selected."),
    }
}
*/