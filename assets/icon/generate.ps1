# Generate PNG + ICO for KebiControl — stylized K + voice dot.
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

    # Gradient #5B9BFF -> #3B82F6
    $rect = New-Object System.Drawing.Rectangle 0, 0, $size, $size
    $grad = New-Object System.Drawing.Drawing2D.LinearGradientBrush $rect, ([System.Drawing.Color]::FromArgb(91, 155, 255)), ([System.Drawing.Color]::FromArgb(59, 130, 246)), 90
    $g.FillRectangle($grad, $rect)
    $grad.Dispose()

    $s = $size / 128.0

    # Vertical bar
    $g.FillRectangle([System.Drawing.Brushes]::White, [int](20*$s), [int](20*$s), [int](22*$s), [int](88*$s))
    # Upper diagonal
    $upper = @(
        [System.Drawing.Point]::new([int](42*$s), [int](68*$s)),
        [System.Drawing.Point]::new([int](70*$s), [int](32*$s)),
        [System.Drawing.Point]::new([int](90*$s), [int](32*$s)),
        [System.Drawing.Point]::new([int](60*$s), [int](68*$s))
    )
    $g.FillPolygon([System.Drawing.Brushes]::White, $upper)
    # Lower diagonal
    $lower = @(
        [System.Drawing.Point]::new([int](42*$s), [int](68*$s)),
        [System.Drawing.Point]::new([int](70*$s), [int](104*$s)),
        [System.Drawing.Point]::new([int](90*$s), [int](104*$s)),
        [System.Drawing.Point]::new([int](60*$s), [int](68*$s))
    )
    $g.FillPolygon([System.Drawing.Brushes]::White, $lower)
    # Voice dot
    $dotR = [int](9*$s)
    if ($dotR -lt 2) { $dotR = 2 }
    $g.FillEllipse([System.Drawing.Brushes]::White, [int](106*$s) - $dotR, [int](32*$s) - $dotR, $dotR*2, $dotR*2)

    $g.Dispose()
    return $bmp
}

$bmp = New-KebiLogo -size 512
$bmp.Save($pngPath, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose()

$sizes = @(16, 24, 32, 48, 64, 128, 256)
$bitmaps = @()
foreach ($s in $sizes) { $bitmaps += ,(New-KebiLogo -size $s) }

$icoStream = New-Object System.IO.MemoryStream
$writer = New-Object System.IO.BinaryWriter $icoStream
$writer.Write([uint16]0); $writer.Write([uint16]1); $writer.Write([uint16]$bitmaps.Count)
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
