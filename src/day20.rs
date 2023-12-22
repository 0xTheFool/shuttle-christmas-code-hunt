use axum::{body::Bytes, debug_handler};
use git2::Repository;
use std::io::Cursor ;
use tar::Archive;

#[debug_handler]
pub async fn get_archive_files(body: Bytes) -> &'static str {
    let bytes: Vec<_> = body.iter().map(|x| *x).collect();

    Archive::new(Cursor::new(bytes))
        .entries()
        .expect("Could not Parse Archive")
        .count()
        .to_string()
        .leak()
}

#[debug_handler]
pub async fn get_archive_files_size(body: Bytes) -> &'static str {
    let bytes: Vec<_> = body.iter().map(|x| *x).collect();

    Archive::new(Cursor::new(bytes))
        .entries()
        .expect("Could not Parse Archive")
        .flatten()
        .map(|f| f.size() )
        .sum::<u64>()
        .to_string()
        .leak()
}

#[debug_handler]
pub async fn task2_cookie(body: Bytes) -> &'static str {
    let bytes: Vec<_> = body.iter().map(|x| *x).collect();
    
    let tempdir = tempfile::tempdir().unwrap();
    Archive::new(Cursor::new(bytes))
        .unpack(tempdir.path()).unwrap();
  
    let repo = Repository::open(tempdir.path()).unwrap();

    let commit = repo.find_branch("christmas", git2::BranchType::Local)
        .expect("Branch doesnot exist")
        .get()
        .peel_to_commit()
        .expect("Could not extract commit");


    let cookie  = cookiefinder(0, &commit, &repo).1.unwrap();


    format!("{} {}",cookie.author().name().unwrap_or_default(), cookie.id()).leak()
}


fn cookiefinder<'a>(count: u32,commit: &git2::Commit<'a>,repo: &git2::Repository) 
-> (u32, Option<git2::Commit<'a>>) {

    let tree = commit.tree().unwrap();
    let mut santa = Vec::new();

    tree.walk(git2::TreeWalkMode::PreOrder, |_,e| {
        if e.name() == Some("santa.txt") {
            santa.push(e.clone().to_object(&repo).expect("Could not push object"));
        }
        git2::TreeWalkResult::Ok
    }).unwrap();

    let strings: Vec<&str> = santa
        .iter()
        .filter_map(|o| o.as_blob())
        .map(git2::Blob::content)
        .flat_map(|c| std::str::from_utf8(c) )
        .filter(|s| s.contains("COOKIE"))
        .collect();

    if !strings.is_empty() {
        return (count, Some(commit.clone()));
    }

    commit.parents()
        .map(|p| cookiefinder(count + 1, &p, repo))
        .min_by_key(|c| c.0)
        .unwrap_or((0,None))
}
