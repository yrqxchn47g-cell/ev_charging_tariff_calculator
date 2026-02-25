# EV Charging Tariff Calculator – Hilfe

## Übersicht
Der EV Charging Tariff Calculator hilft Ihnen, verschiedene Ladetarife
für Elektrofahrzeuge zu vergleichen und den Breakeven-Punkt zu ermitteln.

## Funktionen

### Tarifvergleich
- Geben Sie den **Preis pro kWh** und die **monatliche Grundgebühr** ein
- Vergleichen Sie bis zu zwei Tarife gleichzeitig

### Breakeven-Analyse
- Ermittelt ab welcher **monatlichen Fahrleistung (km)** sich ein Tarif
  mit niedrigerem kWh-Preis aber höherer Grundgebühr lohnt
- Berücksichtigt den **Verbrauch (kWh/100km)** Ihres Fahrzeugs

### Kostendiagramm
- Visualisiert die monatlichen Kosten beider Tarife als Kurvendiagramm
- Der Schnittpunkt zeigt den Breakeven-Punkt

## Bedienung
1. Starten Sie die Anwendung
2. Geben Sie die Tarifdaten ein (Name, Preis pro kWh, monatliche Grundgebühr)
3. Klicken Sie auf **"Calculate Tariff"** zur Berechnung
4. Das Ergebnis zeigt die monatlichen Kosten und den Breakeven-Punkt

## Begriffe

| Begriff | Beschreibung |
|---------|-------------|
| **kWh** | Kilowattstunde – Maßeinheit für elektrische Energie |
| **Preis pro kWh** | Kosten pro verbrauchter Kilowattstunde beim Laden |
| **Grundgebühr** | Monatliche Fixkosten eines Tarifs |
| **Breakeven** | Punkt, ab dem ein Tarif günstiger wird als ein anderer |
| **Verbrauch** | Energieverbrauch des Fahrzeugs in kWh pro 100 km |

## Beispiel
- **Tarif A**: 0,30 EUR/kWh, 0 EUR Grundgebühr
- **Tarif B**: 0,20 EUR/kWh, 15 EUR Grundgebühr
- **Verbrauch**: 16 kWh/100km
- **Breakeven**: Ab ca. 938 km/Monat ist Tarif B günstiger

## Hinweise
- Die PDF-Hilfedatei kann jederzeit über den **"Hilfe"**-Button in der Anwendung geöffnet werden.
- Zur Erstellung der PDF-Version dieser Datei verwenden Sie: `pandoc docs/hilfe.md -o docs/hilfe.pdf`

## Kontakt
Bei Fragen wenden Sie sich an: you@example.com
