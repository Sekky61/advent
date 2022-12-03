# setup for new day

import os
import sys
import requests
from dotenv import load_dotenv, find_dotenv

def download_and_save_input(year, day):
    yeartext = year_to_text(year)
    input_text = download_input(year, day)

    with open(f"inputs/{yeartext}/day{day}", "a") as input_file:
        input_file.write(input_text)

def download_input(year, day):
    # Set up the request URL
    fullyear = year_to_fullyear(year)
    url = f"https://adventofcode.com/{fullyear}/day/{day}/input"
    session_id = os.environ["AOC_SESSION_ID"]

    print(url)

    # Set the request headers
    headers = {
        "Cookie": f"session={session_id}",
    }

    # Send the request and get the response
    response = requests.get(url, headers=headers)

    # Return the response text
    return response.text

def year_to_text(year: int):
    texts = ["twenty", "twentyone", "twentytwo", "twentythree", "twentyfour"]
    return texts[year - 20]

def year_to_fullyear(year: int):
    if year < 30:
        return 2000 + year
    else:
        return year

def copy_file(filename, copy_name):
    # Open the original file in read mode
    with open(filename, "r") as original_file:
        # Read the entire contents of the original file
        original_file_contents = original_file.read()
    
    # you can format it here with original_file_contents.format()

    # Open a new file in write mode
    with open(copy_name, "w") as copy_file:
        # Write the contents of the original file to the new file
        copy_file.write(original_file_contents)

def main():
    load_dotenv(find_dotenv())

    if len(sys.argv) != 3:
        print("Usage:\tday_setup.py [year] [day]")
        print("Example:\tday_setup.py 22 1")
        return

    year = int(sys.argv[1])
    day = int(sys.argv[2])

    if year > 2000:
        year -= 2000

    if year < 0 or year > 30:
        raise ValueError(f"Year value {year} not valid")
    
    if day < 0 or day > 31:
        raise ValueError(f"Day value {day} not valid")

    yeartext = year_to_text(year)

    download_and_save_input(year, day)

    with open(f"src/{yeartext}/mod.rs", "a") as mod_file:
        mod_file.write(f"#[allow(dead_code)]\npub mod day{day};\n")
    
    copy_file("day_solution.template", f"src/{yeartext}/day{day}.rs")

main()