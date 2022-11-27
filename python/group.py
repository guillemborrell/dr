#!/usr/bin/env python3

import sys
import pandas as pd

df = pd.read_csv(sys.stdin)
print(df.groupby("Dept", sort=False, as_index=False).Weekly_Sales.mean())