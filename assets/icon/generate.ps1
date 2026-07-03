#requires -Version 5.1
# Generates KebiControl.ico from SVG via .NET (System.Drawing).
# Minimalism by KebiLab.

Add-Type -AssemblyName System.Drawing

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$icoPath = Join-Path $root 'kebicontrol.ico'
$pngPath = Join-Path $root 'kebicontrol.png'

# Generate a minimal, modern icon programmatically.
$sizes = @(16, 24, 32, 48, 64, 128, 256)
$bitmaps = @()

function New-KebiIcon([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap $size, $size
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    # Rounded background (we approximate with a path)
    $bgRect = New-Object System.Drawing.Rectangle 0, 0, $size, $size
    $bgPath = New-Object System.Drawing.Drawing2D.GraphicsPath
    $r = [int]($size * 0.22)
    $bgPath.AddArc($bgRect.X, $bgRect.Y, $r * 2, $r * 2, 180, 90)
    $bgPath.AddArc($bgRect.Right - $r * 2, $bgRect.Y, $r * 2, $r * 2, 270, 90)
    $bgPath.AddArc($bgRect.Right - $r * 2, $bgRect.Bottom - $r * 2, $r * 2, $r * 2, 0, 90)
    $bgPath.AddArc($bgRect.X, $bgRect.Bottom - $r * 2, $r * 2, $r * 2, 90, 90)
    $bgPath.CloseFigure()

    # Gradient: #0F172A -> #1E293B
    $bgBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush $bgRect, ([System.Drawing.Color]::FromArgb(15, 23, 42)), ([System.Drawing.Color]::FromArgb(30, 41, 59)), 45
    $g.FillPath($bgBrush, $bgPath)
    $bgBrush.Dispose()

    # Subtle border
    $borderPen = New-Object System.Drawing.Pen ([System.Drawing.Color]::FromArgb(51, 65, 85)), ([Math]::Max(1, $size / 96))
    $g.DrawPath($borderPen, $bgPath)
    $borderPen.Dispose()

    # Letter K (gradient #60A5FA -> #22D3EE)
    $kBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush $bgRect, ([System.Drawing.Color]::FromArgb(96, 165, 250)), ([System.Drawing.Color]::FromArgb(34, 211, 238)), 90
    $fontSize = [Math]::Max(8, [int]($size * 0.62))
    $font = New-Object System.Drawing.Font 'Segoe UI', $fontSize, ([System.Drawing.FontStyle]::Bold), ([System.Drawing.GraphicsUnit]::Pixel)
    $sf = New-Object System.Drawing.StringFormat
    $sf.Alignment = [System.Drawing.StringAlignment]::Center
    $sf.LineAlignment = [System.Drawing.StringAlignment]::Center
    $pt = New-Object System.Drawing.PointF ([float]($size / 2)), ([float]($size / 2))
    $g.DrawString('K', $font, $kBrush, $pt, $sf)
    $font.Dispose()
    $kBrush.Dispose()
    $g.Dispose()
    $bgPath.Dispose()
    return $bmp
}

foreach ($s in $sizes) {
    $b = New-KebiIcon -size $s
    $bitmaps += ,$b
}

# Save primary PNG (256x256)
$bitmaps[-1].Save($pngPath, [System.Drawing.Imaging.ImageFormat]::Png)
Write-Host "PNG: $pngPath"

# Build a multi-size .ico
$icoStream = New-Object System.IO.MemoryStream
$writer = New-Object System.IO.BinaryWriter $icoStream

# ICONDIR
$writer.Write([uint16]0)                # reserved
$writer.Write([uint16]1)                # type = icon
$writer.Write([uint16]$bitmaps.Count)   # count

$pngStreams = @()
foreach ($b in $bitmaps) {
    $ms = New-Object System.IO.MemoryStream
    $b.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $pngStreams += ,$ms
}

# directory size = 6 + 16 * count
$dataOffset = 6 + 16 * $bitmaps.Count
for ($i = 0; $i -lt $bitmaps.Count; $i++) {
    $b = $bitmaps[$i]
    $ms = $pngStreams[$i]
    $bytes = $ms.ToArray()
    $w = if ($b.Width -ge 256) { 0 } else { $b.Width }
    $h = if ($b.Height -ge 256) { 0 } else { $b.Height }
    $writer.Write([byte]$w)
    $writer.Write([byte]$h)
    $writer.Write([byte]0)               # colors in palette
    $writer.Write([byte]0)               # reserved
    $writer.Write([uint16]1)             # planes
    $writer.Write([uint16]32)            # bit count
    $writer.Write([uint32]$bytes.Length) # size
    $writer.Write([uint32]$dataOffset)   # offset
    $dataOffset += $bytes.Length
}

# data
for ($i = 0; $i -lt $bitmaps.Count; $i++) {
    $writer.Write($pngStreams[$i].ToArray())
    $pngStreams[$i].Dispose()
    $bitmaps[$i].Dispose()
}

[System.IO.File]::WriteAllBytes($icoPath, $icoStream.ToArray())
$writer.Close()
$icoStream.Dispose()
Write-Host "ICO: $icoPath"
