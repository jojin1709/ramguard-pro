Add-Type -AssemblyName System.Drawing
$srcPath = 'C:\Users\jojin\.gemini\antigravity-ide\brain\7fd38974-f138-44d9-9cc7-d018c7a22b97\ramguard_installer_splash_1785914113869.png'
$outPath = 'src-tauri\icons\installer_sidebar.bmp'

$src = [System.Drawing.Image]::FromFile($srcPath)
$bmp = New-Object System.Drawing.Bitmap(164, 314, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$g.DrawImage($src, 0, 0, 164, 314)
$g.Dispose()
$src.Dispose()
$bmp.Save($outPath, [System.Drawing.Imaging.ImageFormat]::Bmp)
$bmp.Dispose()
Write-Host "BMP created successfully!"
