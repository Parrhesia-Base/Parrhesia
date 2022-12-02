use anyhow::{bail, Context, Result};
use std::{env, ffi::OsString, fs, str::FromStr};

/// The main setup function
fn main() -> Result<()>
{
    println!( " ==================================================================== " );
    println!( " ==================================================================== " );
    println!( "  ██████   █████  ██████  ██████  ██   ██ ███████ ███████ ██  █████   " );
    println!( "  ██   ██ ██   ██ ██   ██ ██   ██ ██   ██ ██      ██      ██ ██   ██  " );
    println!( "  ██████  ███████ █████   █████   ███████ █████   ███████ ██ ███████  " );
    println!( "  ██      ██   ██ ██  ██  ██  ██  ██   ██ ██           ██ ██ ██   ██  " );
    println!( "  ██      ██   ██ ██   ██ ██   ██ ██   ██ ███████ ███████ ██ ██   ██  " );
    println!( "  V0.1.0              Shared with much love by Joey             2022  " );
    println!( " ==================================================================== " );
    println!( " ==================================================================== " );
    println!( "                             INTRODUCTION                             " );
    println!( " In order to support a first-class cross platform experience, this    " );
    println!( " project features a unique build system. Calling 'cargo run' will     " );
    println!( " compile all project dependencies, and run this setup script.         " );
    println!( "                                                                      " );
    println!( "                                GIZMO                                 " );
    println!( " This setup script compiles the 'gizmo' binary, and copies it to the  " );
    println!( " root directory of this project. Gizmo is to Parrhesia what Artisan   " );
    println!( " is to Laravel. It is a tool to facilitate better interaction between " );
    println!( " developer and project. Once gizmo is available, you should no longer " );
    println!( " work with the cargo build system directly, but instead do all your   " );
    println!( " project management through 'gizmo'. See 'gizmo -h' for a thorough    " );
    println!( " explanation of how it can be used.                                   " );
    

    // println!( " Welcome... To Parrhesia!" );

    let mut exe_dir = env::current_exe().with_context( || "Unable to determine location of target output. Please report this error, as it shouldn't be possible to reach" )?;

    let gizmo_target = 'target: {
        exe_dir.pop();

        for path in fs::read_dir(&exe_dir).with_context( || format!( "Failed to get file list from {:?}", exe_dir ) )?
        {
            let Ok( path ) = path else {
                println!( "Failed to process the following directory entry: {:?}", path );
                continue;
            };
            let name = path.file_name();

            if name != OsString::from_str("gizmo.d").unwrap() // This unwrap can't be reached since the string has to be valid, since it exists
                && name.to_str().unwrap().contains("gizmo")
            {
                let Some( name_str ) = name.to_str() else {
                    println!( "File name '{:?}' contains invalid unicode!", name );
                    continue;
                };

                if name_str.contains( "gizmo" ) {
                    break 'target path;
                }
            }
        }

        bail!("Failed to find the gizmo executable in {:?}! Try rebuilding?", exe_dir);
    }.path();

    let project_gizmo = {
        exe_dir.pop();
        exe_dir.pop();
        exe_dir.push(gizmo_target.file_name().unwrap());
        exe_dir
    };

    // println!("Current target directory: {:?}", gizmo_target);
    // println!("Current project directory: {:?}", project_gizmo);

    let copy = if project_gizmo.exists() {
        let btime = gizmo_target
            .metadata().with_context( || format!( "Failed to get metadata of the following file: {:?}", gizmo_target ) )?
            .modified().with_context( || format!( "Failed to get modified time of the following file: {:?}", gizmo_target ) )?;
        
        let ptime = project_gizmo
            .metadata().with_context( || format!( "Failed to get metadata of the following file: {:?}", project_gizmo ) )?
            .modified().with_context( || format!( "Failed to get modified time of the following file: {:?}", project_gizmo ) )?;

        if btime > ptime {
            true
        } else {
            false
        }
    } else {
        true
    };

    if copy {
        fs::copy( &gizmo_target, &project_gizmo ).with_context( || format!( "Failed to copy {:?} to {:?}", gizmo_target, project_gizmo ) )?;
        println!( "Moved gizmo to the project directory" );
    }

    Ok(())
}
