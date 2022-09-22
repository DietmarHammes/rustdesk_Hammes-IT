lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Ihr Desktop"),
        ("desk_tip", "Mit dieser ID und diesem Passwort können Sie auf Ihren Desktop zugreifen."),
        ("Password", "Passwort"),
        ("Ready", "Bereit"),
        ("Established", "Verbunden"),
        ("connecting_status", "Verbinden mit dem RustDesk-Netzwerk..."),
        ("Enable Service", "Verbindungsserver aktivieren"),
        ("Start Service", "Starte Vermittlungsdienst"),
        ("Service is running", "Vermittlungsdienst aktiv"),
        ("Service is not running", "Vermittlungsdienst deaktiviert"),
        ("not_ready_status", "Nicht bereit. Bitte überprüfen Sie Ihre Verbindung"),
        ("Control Remote Desktop", "Entfernten PC steuern"),
        ("Transfer File", "Datei übertragen"),
        ("Connect", "Verbinden"),
        ("Recent Sessions", "Letzte Sitzungen"),
        ("Address Book", "Adressbuch"),
        ("Confirmation", "Bestätigung"),
        ("TCP Tunneling", "TCP Tunneln"),
        ("Remove", "Entfernen"),
        ("Refresh random password", "Neues zufälliges Passwort"),
        ("Set your own password", "Eigenes Passwort setzen"),
        ("Enable Keyboard/Mouse", "Tastatur/Maus aktivieren"),
        ("Enable Clipboard", "Zwischenablage aktivieren"),
        ("Enable File Transfer", "Dateiübertragung aktivieren"),
        ("Enable TCP Tunneling", "TCP-Tunneln aktivieren"),
        ("IP Whitelisting", "IP-Whitelist"),
        ("ID/Relay Server", "ID/Vermittlungsserver"),
        ("Stop service", "Vermittlungsdienst deaktivieren"),
        ("Change ID", "ID ändern"),
        ("Website", "Webseite"),
        ("About", "Über"),
        ("Mute", "Stummschalten"),
        ("Audio Input", "Audio-Eingang"),
        ("Enhancements", "Verbesserungen"),
        ("Hardware Codec", "Hardware-Codec"),
        ("Adaptive Bitrate", "Adaptive Bitrate"),
        ("ID Server", "ID Server"),
        ("Relay Server", "Vermittlungsserver"),
        ("API Server", "API-Server"),
        ("invalid_http", "Muss mit http:// oder https:// beginnen"),
        ("Invalid IP", "Ungültige IP-Adresse"),
        ("id_change_tip", "Nur die Zeichen a-z, A-Z, 0-9 und _ (Unterstrich) sind erlaubt. Der erste Buchstabe muss a-z, A-Z sein, Länge zwischen 6 und 16."),
        ("Invalid format", "Ungültiges Format"),
        ("server_not_support", "Diese Funktion wird noch nicht vom Server unterstützt"),
        ("Not available", "Nicht verfügbar"),
        ("Too frequent", "Zu häufig"),
        ("Cancel", "Abbrechen"),
        ("Skip", "Überspringen"),
        ("Close", "Schließen"),
        ("Retry", "Erneut versuchen"),
        ("OK", "OK"),
        ("Password Required", "Passwort erforderlich"),
        ("Please enter your password", "Bitte geben Sie das Passwort des entfernten PCs ein."),
        ("Remember password", "Passwort merken"),
        ("Wrong Password", "Falsches Passwort"),
        ("Do you want to enter again?", "Erneut verbinden?"),
        ("Connection Error", "Verbindungsfehler"),
        ("Error", "Fehler"),
        ("Reset by the peer", "Verbindung wurde von der Gegenstelle zurückgesetzt"),
        ("Connecting...", "Verbinden..."),
        ("Connection in progress. Please wait.", "Die Verbindung wird hergestellt. Bitte warten Sie."),
        ("Please try 1 minute later", "Bitte versuchen Sie es später erneut"),
        ("Login Error", "Anmeldefehler"),
        ("Successful", "Erfolgreich"),
        ("Connected, waiting for image...", "Verbunden, warten auf Bild..."),
        ("Name", "Name"),
        ("Type", "Typ"),
        ("Modified", "Geändert"),
        ("Size", "Größe"),
        ("Show Hidden Files", "Versteckte Dateien anzeigen"),
        ("Receive", "Empfangen"),
        ("Send", "Senden"),
        ("Refresh File", "Datei aktualisieren"),
        ("Local", "Lokal"),
        ("Remote", "Entfernt"),
        ("Remote Computer", "Entfernter Computer"),
        ("Local Computer", "Dieser Computer"),
        ("Confirm Delete", "Löschen bestätigen"),
        ("Delete", "Löschen"),
        ("Properties", "Eigenschaften"),
        ("Multi Select", "Mehrfachauswahl"),
        ("Empty Directory", "Leerer Ordner"),
        ("Not an empty directory", "Ordner nicht leer"),
        ("Are you sure you want to delete this file?", "Sind Sie sicher, dass Sie diese Datei löschen wollen?"),
        ("Are you sure you want to delete this empty directory?", "Sind Sie sicher, dass Sie diesen leeren Ordner löschen möchten?"),
        ("Are you sure you want to delete the file of this directory?", "Sind Sie sicher, dass Sie die Datei dieses Ordners löschen möchten?"),
        ("Do this for all conflicts", "Für alle Konflikte merken"),
        ("This is irreversible!", "Dies ist irreversibel!"),
        ("Deleting", "Löschen"),
        ("files", "Dateien"),
        ("Waiting", "Warten"),
        ("Finished", "Fertiggestellt"),
        ("Speed", "Geschwindigkeit"),
        ("Custom Image Quality", "Benutzerdefinierte Bildqualität"),
        ("Privacy mode", "Datenschutz-Modus"),
        ("Block user input", "Benutzereingaben blockieren"),
        ("Unblock user input", "Benutzereingaben freigeben"),
        ("Adjust Window", "Fenster anpassen"),
        ("Original", "Original"),
        ("Shrink", "Verkleinern"),
        ("Stretch", "Strecken"),
        ("Scrollbar", "Scrollleiste"),
        ("ScrollAuto", "Automatisch scrollen"),
        ("Good image quality", "Schöner"),
        ("Balanced", "Ausgeglichen"),
        ("Optimize reaction time", "Schneller"),
        ("Custom", "Benutzerdefiniert"),
        ("Show remote cursor", "Entfernten Cursor anzeigen"),
        ("Show quality monitor", "Qualitätsüberwachung anzeigen"),
        ("Disable clipboard", "Zwischenablage deaktivieren"),
        ("Lock after session end", "Sperren nach Sitzungsende"),
        ("Insert", "Einfügen"),
        ("Insert Lock", "Win+L (Sperren) senden"),
        ("Refresh", "Aktualisieren"),
        ("ID does not exist", "Diese ID existiert nicht"),
        ("Failed to connect to rendezvous server", "Verbindung zum Vermittlungsserver fehlgeschlagen"),
        ("Please try later", "Bitte versuchen Sie es später erneut"),
        ("Remote desktop is offline", "Entfernter PC ist offline"),
        ("Key mismatch", "Schlüssel stimmt nicht mit Serverschlüssel überein"),
        ("Timeout", "Zeitüberschreitung"),
        ("Failed to connect to relay server", "Verbindung zum Vermittlungsserver fehlgeschlagen"),
        ("Failed to connect via rendezvous server", "Verbindung über Vermittlungsserver ist fehlgeschlagen"),
        ("Failed to connect via relay server", "Verbindung über Relay-Server ist fehlgeschlagen"),
        ("Failed to make direct connection to remote desktop", "Direkte Verbindung zum entfernten PC fehlgeschlagen"),
        ("Set Password", "Passwort festlegen"),
        ("OS Password", "Betriebssystem-Passwort"),
        ("install_tip", "Aufgrund der UAC kann RustDesk in manchen Fällen nicht ordnungsgemäß auf der Gegenseite funktionieren. Um UAC zu vermeiden, klicken Sie bitte auf die Schaltfläche unten, um RustDesk auf dem System zu installieren"),
        ("Click to upgrade", "Zum Aktualisieren anklicken"),
        ("Click to download", "Zum Herunterladen klicken"),
        ("Click to update", "Zum Aktualisieren klicken"),
        ("Configure", "Konfigurieren"),
        ("config_acc", "Um Ihren PC aus der Ferne zu steuern, müssen Sie RustDesk Zugriffsrechte erteilen."),
        ("config_screen", "Um aus der Ferne auf Ihren PC zugreifen zu können, müssen Sie RustDesk \"Bildschirm-Aufnahme\"-Berechtigung erteilen."),
        ("Installing ...", "Installiere ..."),
        ("Install", "Installieren"),
        ("Installation", "Installation"),
        ("Installation Path", "Installationspfad"),
        ("Create start menu shortcuts", "Verknüpfung im Startmenü erstellen"),
        ("Create desktop icon", "Desktop-Verknüpfung erstellen"),
        ("agreement_tip", "Durch die Installation akzeptieren Sie die Lizenzvereinbarung"),
        ("Accept and Install", "Akzeptieren und Installieren"),
        ("End-user license agreement", "Lizenzvereinbarung für Endbenutzer"),
        ("Generating ...", "Generiere..."),
        ("Your installation is lower version.", "Ihre Installation ist älter."),
        ("not_close_tcp_tip", "Schließen Sie dieses Fenster nicht, solange Sie den Tunnel benutzen."),
        ("Listening ...", "Höre..."),
        ("Remote Host", "Entfernter PC"),
        ("Remote Port", "Entfernter Port"),
        ("Action", "Aktion"),
        ("Add", "Hinzufügen"),
        ("Local Port", "Lokaler Port"),
        ("setup_server_tip", "Für eine schnellere Verbindung richten Sie bitte Ihren eigenen Verbindungsserver ein"),
        ("Too short, at least 6 characters.", "Zu kurz, mindestens 6 Zeichen."),
        ("The confirmation is not identical.", "Die Passwörter sind nicht identisch."),
        ("Permissions", "Berechtigungen"),
        ("Accept", "Akzeptieren"),
        ("Dismiss", "Ablehnen"),
        ("Disconnect", "Verbindung trennen"),
        ("Allow using keyboard and mouse", "Verwendung von Maus und Tastatur zulassen"),
        ("Allow using clipboard", "Verwendung der Zwischenablage zulassen"),
        ("Allow hearing sound", "System-Audio übertragen"),
        ("Allow file copy and paste", "Kopieren und Einfügen von Dateien zulassen"),
        ("Connected", "Verbunden"),
        ("Direct and encrypted connection", "Direkte und verschlüsselte Verbindung"),
        ("Relayed and encrypted connection", "Vermittelte und verschlüsselte Verbindung"),
        ("Direct and unencrypted connection", "Direkte und unverschlüsselte Verbindung"),
        ("Relayed and unencrypted connection", "Vermittelte und unverschlüsselte Verbindung"),
        ("Enter Remote ID", "Remote-ID eingeben"),
        ("Enter your password", "Geben Sie Ihr Passwort ein"),
        ("Logging in...", "Anmeldung..."),
        ("Enable RDP session sharing", "RDP-Sitzungsfreigabe aktivieren"),
        ("Auto Login", "Automatisch anmelden (nur gültig, wenn Sie \"Sperren nach Sitzungsende\" aktiviert haben)"),
        ("Enable Direct IP Access", "Direkten IP-Zugang aktivieren"),
        ("Rename", "Umbenennen"),
        ("Space", "Speicherplatz"),
        ("Create Desktop Shortcut", "Desktop-Verknüpfung erstellen"),
        ("Change Path", "Pfad ändern"),
        ("Create Folder", "Ordner erstellen"),
        ("Please enter the folder name", "Bitte geben Sie den Ordnernamen ein"),
        ("Fix it", "Reparieren"),
        ("Warning", "Warnung"),
        ("Login screen using Wayland is not supported", "Anmeldebildschirm wird mit Wayland nicht unterstützt"),
        ("Reboot required", "Neustart erforderlich"),
        ("Unsupported display server ", "Nicht unterstützter Display-Server"),
        ("x11 expected", "X11 erwartet"),
        ("Port", "Port"),
        ("Settings", "Einstellungen"),
        ("Username", " Benutzername"),
        ("Invalid port", "Ungültiger Port"),
        ("Closed manually by the peer", "Von der Gegenstelle manuell geschlossen"),
        ("Enable remote configuration modification", "Änderung der Konfiguration aus der Ferne zulassen"),
        ("Run without install", "Ohne Installation ausführen"),
        ("Always connected via relay", "Immer über Relay-Server verbunden"),
        ("Always connect via relay", "Immer über Relay-Server verbinden"),
        ("whitelist_tip", "Nur IPs auf der Whitelist können zugreifen"),
        ("Login", "Anmelden"),
        ("Logout", "Abmelden"),
        ("Tags", "Stichworte"),
        ("Search ID", "Suche ID"),
        ("Current Wayland display server is not supported", "Der aktuelle Wayland-Anzeigeserver wird nicht unterstützt"),
        ("whitelist_sep", "Getrennt durch Komma, Semikolon, Leerzeichen oder Zeilenumbruch"),
        ("Add ID", "ID hinzufügen"),
        ("Add Tag", "Stichwort hinzufügen"),
        ("Unselect all tags", "Alle Stichworte abwählen"),
        ("Network error", "Netzwerkfehler"),
        ("Username missed", "Benutzername vergessen"),
        ("Password missed", "Passwort vergessen"),
        ("Wrong credentials", "Falsche Anmeldedaten"),
        ("Edit Tag", "Stichwort bearbeiten"),
        ("Unremember Password", "Passwort vergessen"),
        ("Favorites", "Favoriten"),
        ("Add to Favorites", "Zu Favoriten hinzufügen"),
        ("Remove from Favorites", "Aus Favoriten entfernen"),
        ("Empty", "Leer"),
        ("Invalid folder name", "Ungültiger Ordnername"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Rechnername"),
        ("Discovered", "Gefunden"),
        ("install_daemon_tip", "Um mit System zu starten, muss der Systemdienst installiert sein"),
        ("Remote ID", "Entfernte ID"),
        ("Paste", "Einfügen"),
        ("Paste here?", "Hier einfügen?"),
        ("Are you sure to close the connection?", "Möchten Sie diese Verbindung wirklich trennen?"),
        ("Download new version", "Neue Version herunterladen"),
        ("Touch mode", "Touch-Modus"),
        ("Mouse mode", "Maus-Modus"),
        ("One-Finger Tap", "1-Finger-Tipp"),
        ("Left Mouse", "Linksklick"),
        ("One-Long Tap", "1-Finger-Halten"),
        ("Two-Finger Tap", "2-Finger-Tipp"),
        ("Right Mouse", "Rechtsklick"),
        ("One-Finger Move", "Einen Finger bewegen"),
        ("Double Tap & Move", "Doppeltippen und bewegen"),
        ("Mouse Drag", "Maus bewegen"),
        ("Three-Finger vertically", "Drei Finger vertikal bewegen"),
        ("Mouse Wheel", "Mausrad"),
        ("Two-Finger Move", "Zwei Finger bewegen"),
        ("Canvas Move", "Sichtfeld bewegen"),
        ("Pinch to Zoom", "2-Finger-Zoom"),
        ("Canvas Zoom", "Sichtfeld-Zoom"),
        ("Reset canvas", "Sichtfeld zurücksetzen"),
        ("No permission of file transfer", "Keine Dateizugriff-Berechtigung"),
        ("Note", "Anmerkung"),
        ("Connection", "Verbindung"),
        ("Share Screen", "Bildschirm freigeben"),
        ("CLOSE", "DEAKTIV."),
        ("OPEN", "AKTIVIER."),
        ("Chat", "Chat"),
        ("Total", "Gesamt"),
        ("items", "Einträge"),
        ("Selected", "Ausgewählt"),
        ("Screen Capture", "Bildschirmzugr."),
        ("Input Control", "Eingabezugriff"),
        ("Audio Capture", "Audiozugriff"),
        ("File Connection", "Dateizugriff"),
        ("Screen Connection", "Bildschirmanschluss"),
        ("Do you accept?", "Verbindung zulassen?"),
        ("Open System Setting", "Systemeinstellung öffnen"),
        ("How to get Android input permission?", "Wie erhalte ich eine Android-Eingabeberechtigung?"),
        ("android_input_permission_tip1", "Damit ein Remote-Gerät Ihr Android-Gerät steuern kann, müssen Sie RustDesk erlauben, den Dienst \"Barrierefreiheit\" zu verwenden."),
        ("android_input_permission_tip2", "Bitte gehen Sie zur nächsten Systemeinstellungsseite, suchen und geben Sie [Installierte Dienste] ein, schalten Sie den Dienst [RustDesk Input] ein."),
        ("android_new_connection_tip", "möchte ihr Gerät steuern."),
        ("android_service_will_start_tip", "Durch das Aktivieren der Bildschirmfreigabe wird der Dienst automatisch gestartet, sodass andere Geräte dieses Android-Gerät steuern können."),
        ("android_stop_service_tip", "Durch das Deaktivieren des Dienstes werden automatisch alle hergestellten Verbindungen getrennt."),
        ("android_version_audio_tip", "Ihre Android-Version unterstützt keine Audioaufnahme, bitte aktualisieren Sie auf Android 10 oder höher, falls möglich."),
        ("android_start_service_tip", "Tippen Sie auf [Dienst aktivieren] oder aktivieren Sie die Berechtigung [Bildschirmzugr.], um den Bildschirmfreigabedienst zu starten."),
        ("Account", "Konto"),
        ("Overwrite", "Überschreiben"),
        ("This file exists, skip or overwrite this file?", "Diese Datei existiert; überspringen oder überschreiben?"),
        ("Quit", "Beenden"),
        ("doc_mac_permission", "https://rustdesk.com/docs/de/manual/mac/#berechtigungen-aktivieren"),
        ("Help", "Hilfe"),
        ("Failed", "Fehlgeschlagen"),
        ("Succeeded", "Erfolgreich"),
        ("Someone turns on privacy mode, exit", "Jemand hat den Datenschutzmodus aktiviert, beende..."),
        ("Unsupported", "Nicht unterstützt"),
        ("Peer denied", "Die Gegenstelle hat die Verbindung abgelehnt"),
        ("Please install plugins", "Bitte installieren Sie Plugins"),
        ("Peer exit", "Die Gegenstelle hat die Verbindung getrennt"),
        ("Failed to turn off", "Ausschalten fehlgeschlagen"),
        ("Turned off", "Ausgeschaltet"),
        ("In privacy mode", "Datenschutzmodus aktivieren"),
        ("Out privacy mode", "Datenschutzmodus deaktivieren"),
        ("Language", "Sprache"),
        ("Keep RustDesk background service", "RustDesk im Hintergrund ausführen"),
        ("Ignore Battery Optimizations", "Batterieoptimierung ignorieren"),
        ("android_open_battery_optimizations_tip", "Möchten Sie die Batterieopimierungs-Einstellungen öffnen?"),
        ("Connection not allowed", "Verbindung abgelehnt"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use temporary password", "Temporäres Passwort verwenden"),
        ("Use permanent password", "Dauerhaftes Passwort verwenden"),
        ("Use both passwords", "Beide Passwörter verwenden"),
        ("Set permanent password", "Dauerhaftes Passwort setzen"),
        ("Set temporary password length", "Länge des temporären Passworts setzen"),
        ("Enable Remote Restart", "Entfernten Neustart aktivieren"),
        ("Allow remote restart", "Entfernten Neustart erlauben"),
        ("Restart Remote Device", "Entferntes Gerät neu starten"),
        ("Are you sure you want to restart", "Möchten Sie das entfernte Gerät wirklich neu starten?"),
        ("Restarting Remote Device", "Entferntes Gerät wird neu gestartet"),
        ("remote_restarting_tip", "Entferntes Gerät startet neu, bitte schließen Sie diese Meldung und verbinden Sie sich mit dem dauerhaften Passwort erneut."),
        ("Copied", ""),
        ("Exit Fullscreen", "Vollbild beenden"),
        ("Fullscreen", "Ganzer Bildschirm"),
        ("Mobile Actions", "Mobile Aktionen"),
        ("Select Monitor", "Wählen Sie Überwachen aus"),
        ("Control Actions", "Kontrollaktionen"),
        ("Display Settings", "Bildschirmeinstellungen"),
        ("Ratio", "Verhältnis"),
        ("Image Quality", "Bildqualität"),
        ("Scroll Style", "Scroll-Stil"),
        ("Show Menubar", "Menüleiste anzeigen"),
        ("Hide Menubar", "Menüleiste ausblenden"),
        ("Direct Connection", "Direkte Verbindung"),
        ("Relay Connection", "Relaisverbindung"),
        ("Secure Connection", "Sichere Verbindung"),
        ("Insecure Connection", "Unsichere Verbindung"),
        ("Scale original", "Original skalieren"),
        ("Scale adaptive", "Adaptiv skalieren"),
        ("General", ""),
        ("Security", ""),
        ("Acount", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable Audio", ""),
        ("Temporary Password Length", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Port", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Deny remote access", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Enable RDP", ""),
        ("Pin menubar", "Pin-Menüleiste"),
        ("Unpin menubar", "Menüleiste lösen"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", ""),
        ("Allow recording session", ""),
    ].iter().cloned().collect();
}
