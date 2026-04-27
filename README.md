# ICS Converter

Takes a CSV file of events and converts it to an iCalendar file. Includes a lightweight GUI.

## Install

Mac and Windows binaries are available on the [Releases page](https://github.com/ozzythecoder/ie-calendar-view/releases), under "Assets".

### Build from source

Requires [`rustup`](https://rust-lang.org/tools/install/).

1. Download this repo - `git clone https://github.com/ozzythecoder/ie-calendar-view.git`
2. Build for release - `cargo build --release`
3. The built binary will be located under `/target/release/isc-converter`

## Usage

1. From IntelliEvent, download a CSV from the default view.
    - The CSV can be for any timeframe.
    - If you're using a custom view, make sure it includes the following required columns:
        - `Job #`
        - `Job Name`
        - `Prep`
        - `Event Start`
        - `Event End`
        - `De-Prep`
        - `Status`
        - `Create Date`
        - `PO #`
        - `Project #`
    - Other columns will be ignored.
1. Run the program.
1. Click "Choose CSV file" and select your downloaded file.
1. Click "Choose Destination" and choose the folder you want to save the new file to.
1. Enter a unique name in the "Filename" input.
    - This will overwrite any `.ics` file with the same name!
1. Click "Convert!"
    - This will open the new location in your file explorer.

This `.ics` file can now be imported into a calendar of your choosing.

### Suggested workflows

#### Outlook only

First import:

1. Create .ics file
2. Create a blank Outlook calendar
3. Import .ics file into blank calendar

Subsequent imports:

1. Create .ics file
2. Delete all events from the Outlook calendar
    - You can [batch-delete events](https://itstraining.wichita.edu/outlook-delete-old-calendar-events-in-batch-plus-bonus-tip/) from the desktop app. But not from the web app, unfortunately.
3. Import .ics file into the (newly) blank calendar

See `A note on re-importing` for more on this.

### A note on re-importing

I've done my best to account for Outlook's confusing handling of ICS properties, but unfortunately there is no consistent way to guarantee how it handles event updates/duplicates when using direct ICS imports.

In my testing, I've found that importing an event with a new title or time will update the event, but this is not officially supported by Outlook. Additionally, if you make any changes to the description, they will be overwritten if you do another import. For this reason, my official suggestion is to **fully wipe the Outlook calendar and re-import the ICS file** whenever you make significant changes in IntelliEvent. You can do this by either:
- mass-deleting the calendar events (which is easier done in the Outlook desktop app) - this will preserve the calendar and any sharing permissions, or
- deleting and re-creating the calendar.

If this is too much, you can also add the calendar to Apple or Google calendar, who both handle ICS files more gracefully than Outlook. You can then publish this calendar and subscribe to it in Outlook. However, this will create a one-way subscription that you won't be able to edit in Outlook - instead, any changes will need to happen to the original Apple/Google calendar, which will then be pushed to Outlook.
