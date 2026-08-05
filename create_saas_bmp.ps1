Add-Type -AssemblyName System.Drawing

$width = 164
$height = 314

$bmp = New-Object System.Drawing.Bitmap($width, $height, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::ClearTypeGridFit

# 1. Dark Premium Gradient (#0b0e14 to #12151f)
$rect = New-Object System.Drawing.Rectangle(0, 0, $width, $height)
$c1 = [System.Drawing.ColorTranslator]::FromHtml("#0b0e14")
$c2 = [System.Drawing.ColorTranslator]::FromHtml("#161a26")
$bgBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush($rect, $c1, $c2, 90.0)
$g.FillRectangle($bgBrush, $rect)

# 2. Glowing Accent Border (Left & Top Gold Accent)
$goldPen = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#e3c25f"), 3)
$cyanPen = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#3fa9a0"), 2)
$dimPen = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#232838"), 1)

$g.DrawLine($goldPen, 0, 0, $width, 0)
$g.DrawLine($cyanPen, 0, 0, 0, $height)

# 3. Minimal Geometry Background Accents
$circlePen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(18, 227, 194, 95), 2)
$g.DrawEllipse($circlePen, -20, 220, 160, 160)
$g.DrawEllipse($circlePen, 40, -40, 160, 160)

# 4. Top Clean Brand Gauge Ring Graphic
$ringPenTrack = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#232838"), 6)
$ringPenValue = New-Object System.Drawing.Pen([System.Drawing.ColorTranslator]::FromHtml("#e3c25f"), 6)
$ringPenValue.StartCap = [System.Drawing.Drawing2D.LineCap]::Round
$ringPenValue.EndCap = [System.Drawing.Drawing2D.LineCap]::Round

$ringRect = New-Object System.Drawing.Rectangle(47, 45, 70, 70)
$g.DrawArc($ringPenTrack, $ringRect, 140, 260)
$g.DrawArc($ringPenValue, $ringRect, 140, 180)

# Inner Shield Icon
$goldBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#e3c25f"))
$cyanBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#3fa9a0"))
$whiteBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$dimBrush = New-Object System.Drawing.SolidBrush([System.Drawing.ColorTranslator]::FromHtml("#8993a8"))

$fontBrand = New-Object System.Drawing.Font("Sora", 13, [System.Drawing.FontStyle]::Bold)
if (-not $fontBrand) { $fontBrand = New-Object System.Drawing.Font("Segoe UI", 13, [System.Drawing.FontStyle]::Bold) }

$fontPro = New-Object System.Drawing.Font("Segoe UI", 11, [System.Drawing.FontStyle]::Bold)
$fontSub = New-Object System.Drawing.Font("Segoe UI", 8, [System.Drawing.FontStyle]::Regular)
$fontLabel = New-Object System.Drawing.Font("Segoe UI", 7, [System.Drawing.FontStyle]::Bold)
$fontAuthor = New-Object System.Drawing.Font("Segoe UI", 9, [System.Drawing.FontStyle]::Bold)

# 5. Clean Modern Typography
$sf = New-Object System.Drawing.StringFormat
$sf.Alignment = [System.Drawing.StringAlignment]::Center

$g.DrawString("RAMGuard", $fontBrand, $whiteBrush, 82, 135, $sf)
$g.DrawString("PRO", $fontPro, $goldBrush, 82, 160, $sf)
$g.DrawString("Windows RAM Optimizer", $fontSub, $dimBrush, 82, 184, $sf)

# Divider Line
$g.DrawLine($dimPen, 24, 210, 140, 210)

# Author Credit Box
$g.DrawString("DEVELOPED BY", $fontLabel, $cyanBrush, 82, 225, $sf)
$g.DrawString("JOJIN JOHN", $fontAuthor, $goldBrush, 82, 240, $sf)

# Bottom Status Pill
$g.DrawString("🛡️ All Rights Reserved", $fontSub, $dimBrush, 82, 280, $sf)

# Clean up resources
$bgBrush.Dispose()
$goldPen.Dispose()
$cyanPen.Dispose()
$dimPen.Dispose()
$circlePen.Dispose()
$ringPenTrack.Dispose()
$ringPenValue.Dispose()
$goldBrush.Dispose()
$cyanBrush.Dispose()
$whiteBrush.Dispose()
$dimBrush.Dispose()

$bmp.Save("src-tauri\icons\installer_sidebar.bmp", [System.Drawing.Imaging.ImageFormat]::Bmp)
$bmp.Dispose()
Write-Host "Clean SaaS Installer Graphic generated!"
