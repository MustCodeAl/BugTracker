// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


use std::io::Write;





// todo add tauri
#[derive(Debug, Default)]
enum BugStatus {
    #[default]
    New,
    Assigned,
    Reopened,
    Resolved,
    Verified,
    Closed,
}


#[derive(Debug, Default)]
pub struct BugDetail {
    status: BugStatus,
    alias: String,
    product: String,
    component: String,
    version: String,
    platform: String,
    linux: String,
    target_milestone: String,
    assignee: String,
    url: String,
    keywords: String,
    depends_on: String,
    blocks: String,
    reported: String,
    modified: String,
    see_also: String,
    latest_commit: String,
    version_fixed: String,
}

impl BugDetail {
    pub(crate) fn is_resolved(&self) -> bool {
        todo!()
    }
}


#[derive(Default)]
struct BugDetailBuilder
{}

impl BugDetailBuilder {
    fn new() -> BugDetailBuilder {
        {
            BugDetailBuilder {}
        }
    }


    fn build(self) -> BugDetail {
        BugDetail {
            status: BugStatus::New,
            alias: String::new(),
            product: String::new(),
            component: String::new(),
            version: String::new(),
            platform: String::new(),
            linux: String::new(),
            target_milestone: String::new(),
            assignee: String::new(),
            url: String::new(),
            keywords: String::new(),
            depends_on: String::new(),
            blocks: String::new(),
            reported: String::new(),
            modified: String::new(),
            see_also: String::new(),
            latest_commit: String::new(),
            version_fixed: String::new(),
        }
    }
}

// Bugs


impl BugDetail {
    pub fn builder() -> BugDetailBuilder {
        BugDetailBuilder::new()
    }
}


pub fn create_issue() -> BugDetail {
    // todo  show a template to user
    BugDetail::builder().build()
}

fn issue_handler(issue: &BugDetail) {
    // todo  show a template to user
    // match the issue type and status
    // check if its criticicla
    // save results to file



    let mut issue_f = std::fs::File::create("src/resolvedissues.txt").expect("couldnt create file");
    // email users
    issue_f.write_all(format!("{:?}", issue).as_bytes()).expect("couldnt write to file");


    println!("issue: {:?}", issue);
}


pub fn resolve_d_issue(/*&mut self*/issues: Vec<BugDetail>) -> BugDetail {
    for x in issues {


        // do a simple bool check for now
        if x.is_resolved() {
            issue_handler(&x);
        } else {
            println!("issue not resolved yet");

            // add to vec of not resolved issues and save to file
        }
    }


    // todo  show a template to user


    // todo  show a template to user

    BugDetail::builder().build()
}


