# Generate PNG + ICO for KebiControl from a hand-drawn vector.
# Matches user-provided @logo.jpg: gradient blue microphone, white slash.
# Made by KebiLab

Add-Type -AssemblyName System.Drawing

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$icoPath = Join-Path $root 'kebicontrol.ico'
$pngPath = Join-Path $root 'kebicontrol.png'

function New-KebiLogo([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap $size, $size
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    $bgRect = New-Object System.Drawing.Rectangle 0, 0, $size, $size

    # Gradient blue (#1B1B6B -> #2E3BCC -> #3B82F6) vertical
    $grad = New-Object System.Drawing.Drawing2D.LinearGradientBrush $bgRect, ([System.Drawing.Color]::FromArgb(27, 27, 107)), ([System.Drawing.Color]::FromArgb(59, 130, 246)), 135
    $g.FillRectangle($grad, $bgRect)
    $grad.Dispose()

    $scale = $size / 256.0
    $sw = [int](16 * $scale)   # stroke width
    $sw2 = [int](14 * $scale)  # slash width

    # Stand base
    $g.FillRectangle([System.Drawing.Brushes]::White, [int](60*$scale), [int](220*$scale), [int](136*$scale), [int](14*$scale))

    # Vertical stem
    $g.FillRectangle([System.Drawing.Brushes]::White, [int](120*$scale), [int](180*$scale), [int](16*$scale), [int](40*$scale))

    # U-shape arc (around capsule)
    $arcPen = New-Object System.Drawing.Pen ([System.Drawing.Color]::White), $sw
    $arcPen.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $arcPen.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
    $g.DrawArc($arcPen, [int](60*$scale), [int](100*$scale), [int](136*$scale), [int](120*$scale), 180, 180)
    $arcPen.Dispose()

    # Microphone capsule
    $g.FillRectangle([System.Drawing.Brushes]::White, [int](100*$scale), [int](40*$scale), [int](56*$scale), [int](116*$scale))

    # Rounded capsule corners
    $cap = New-Object System.Drawing.Rectangle ([int](100*$scale)), ([int](40*$scale)), ([int](56*$scale)), ([int](116*$scale))
    $capBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::White)
    $g.FillRectangle($capBrush, $cap)
    $capBrush.Dispose()

    # Diagonal slash (white)
    $slash = New-Object System.Drawing.Pen ([System.Drawing.Color]::White), $sw2
    $slash.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
    $slash.EndCap = [System.Drawing.Drawing2D.LineCap]::Round
    $g.DrawLine($slash, [int](92*$scale), [int](64*$scale), [int](164*$scale), [int](184*$scale))
    $slash.Dispose()

    $g.Dispose()
    return $bmp
}

# Save primary PNG
$bmp = New-KebiLogo -size 512
$bmp.Save($pngPath, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose()

# Build multi-size ICO
$sizes = @(16, 24, 32, 48, 64, 128, 256)
$bitmaps = @()
foreach ($s in $sizes) { $bitmaps += ,(New-KebiLogo -size $s) }

$icoStream = New-Object System.IO.MemoryStream
$writer = New-Object System.IO.BinaryWriter $icoStream
$writer.Write([uint16]0)
$writer.Write([uint16]1)
$writer.Write([uint16]$bitmaps.Count)

$pngStreams = @()
foreach ($b in $bitmaps) {
    $ms = New-Object System.IO.MemoryStream
    $b.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $pngStreams += ,$ms
}

$dataOffset = 6 + 16 * $bitmaps.Count
for ($i = 0; $i -lt $bitmaps.Count; $i++) {
    $b = $bitmaps[$i]; $ms = $pngStreams[$i]; $bytes = $ms.ToArray()
    $w = if ($b.Width -ge 256) { 0 } else { $b.Width }
    $h = if ($b.Height -ge 256) { 0 } else { $b.Height }
    $writer.Write([byte]$w); $writer.Write([byte]$h)
    $writer.Write([byte]0); $writer.Write([byte]0)
    $writer.Write([uint16]1); $writer.Write([uint16]32)
    $writer.Write([uint32]$bytes.Length); $writer.Write([uint32]$dataOffset)
    $dataOffset += $bytes.Length
}
for ($i = 0; $i -lt $bitmaps.Count; $i++) {
    $writer.Write($pngStreams[$i].ToArray())
    $pngStreams[$i].Dispose()
    $bitmaps[$i].Dispose()
}
[System.IO.File]::WriteAllBytes($icoPath, $icoStream.ToArray())
$writer.Close(); $icoStream.Dispose()
Write-Host "ICO: $icoPath"
Write-Host "PNG: $pngPath"
