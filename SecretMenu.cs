using BepInEx;
using BepInEx.Unity.IL2CPP;
using Il2CppInterop.Runtime.Injection;
using UnityEngine;

public static class PluginInfo
{
    public const string PLUGIN_GUID = "SecretMenu";
    public const string PLUGIN_NAME = "Secret Menu Unlocker";
    public const string PLUGIN_VERSION = "0.0.1";

    public static PluginLoader? Instance = null;
}

[BepInPlugin("me.axd1x8a.miside.secretmenu", PluginInfo.PLUGIN_NAME, PluginInfo.PLUGIN_VERSION)]
public class PluginLoader : BasePlugin
{
    public PluginLoader() { }

    public override void Load()
    {
        PluginInfo.Instance = this;
        IL2CPPChainloader.AddUnityComponent(typeof(MiSideSecretMenu));
    }
}


public class MiSideSecretMenu : MonoBehaviour
{
    [System.Runtime.InteropServices.DllImport("secretmenu_native.dll")]
    private static extern System.IntPtr Patch();

    private void Start()
    {
        Patch();
    }
}
