#!/usr/bin/env python

import pandas
import spiceypy as spice

import kalast

import setup


spice.kclear()
spice.furnsh("/Users/gregoireh/data/spice/dart/mk/d520_v03.tm")

# et_start = spice.str2et("2022-12-14 08:00:00")
# et_end = spice.str2et("2022-12-15 08:00:00")

et_start = spice.str2et("2022-12-15 08:00:00")
et_end = spice.str2et("2022-12-16 08:00:00")

et = et_start
dt = 60

events = []

oc = 0
ec = 0

last_mo = "08"

while True:
    date = spice.timout(et, kalast.util.TIMOUT3)

    ocid = spice.occult(
        "didymos",
        "ellipsoid",
        "didymos_fixed",
        "dimorphos",
        "ellipsoid",
        "dimorphos_fixed",
        "none",
        "earth",
        et,
    )

    ecid = spice.occult(
        "didymos",
        "ellipsoid",
        "didymos_fixed",
        "dimorphos",
        "ellipsoid",
        "dimorphos_fixed",
        "none",
        "sun",
        et,
    )

    if ocid != oc:
        events.append([et, date, setup.OCID_NAME[ocid]])
        oc = ocid

    if ecid != ec:
        events.append([et, date, setup.ECID_NAME[ecid]])
        ec = ecid

    et += dt

    if et > et_end:
        break

    mo = date[4:6]
    if mo != last_mo:
        print(f"current month: {mo}")
        last_mo = mo

spice.kclear()

df = {}
df["et"] = [et for (et, _, _) in events]
df["date"] = [date for (_, date, _) in events]
df["event"] = [event for (_, _, event) in events]
df = pandas.DataFrame(df)
df.to_csv("out/events_spice.csv", index=False, encoding="utf-8-sig")
