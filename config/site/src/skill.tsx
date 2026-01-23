import React, { useState, useEffect } from 'react';
import { Save, X, ChevronDown, ChevronRight, Settings, Database, AlertCircle, Package, User, Code, Shield, Zap } from 'lucide-react';

// Function to fetch settings - replace empty object with your API call
const fetchSettings = async () => {
  // TODO: Replace with actual API call
  // const response = await fetch('YOUR_API_ENDPOINT');
  // return await response.json();

  // Example data structure (remove when implementing API)
  return {
    skill: {
      id: "light_control",
      name: "Light Control",
      description: "A skill to control the lights",
      entry: "main.avi",
      capabilities: ["intent:light.turn_on", "intent:light.turn_off"],
      permissions: ["speak", "display"],
      author: "Avi Labs",
      version: "1.0.0"
    },

    "settings": [
      {
        "name": "welcome_brightness",
        "setting": {
          "min": 0,
          "max": 100,
          "description": "Brightness level for the welcome light",
          "value": 80,
          "required": null,
          "ui": null,
          "enum_": null,
          "advanced": null,
          "group": null,
          "vtype": "number"
        }
      },
      {
        "name": "linked_devices",
        "setting": {
          "required": null,
          "vtype": "list",
          "description": "Devices to activate together",
          "advanced": null,
          "ui": null,
          "max": null,
          "group": null,
          "min": null,
          "value": [
            "light-bedroom",
            "speaker-bedroom"
          ],
          "enum_": null
        }
      },
      {
        "name": "api_token",
        "setting": {
          "description": "Private API token for debug operations",
          "advanced": true,
          "value": "",
          "min": null,
          "vtype": "string",
          "ui": "password",
          "group": null,
          "max": null,
          "enum_": null,
          "required": null
        }
      },
      {
        "name": "poll_interval",
        "setting": {
          "required": null,
          "vtype": "time.seconds",
          "max": 300,
          "description": "Polling frequency to check if the device is online",
          "min": 5,
          "advanced": null,
          "enum_": null,
          "value": 30,
          "group": null,
          "ui": "slider"
        }
      },
      {
        "name": "device_ip",
        "setting": {
          "enum_": null,
          "description": "IP address of the smart plug",
          "group": null,
          "value": "192.168.1.123",
          "ui": "text",
          "max": null,
          "min": null,
          "vtype": "io.ip",
          "required": true,
          "advanced": null
        }
      },
      {
        "name": "mode",
        "setting": {
          "ui": "dropdown",
          "description": "Select device mode",
          "min": null,
          "advanced": null,
          "required": null,
          "enum_": ["eco", "normal", "turbo"],
          "max": null,
          "group": null,
          "vtype": "enum",
          "value": "eco"
        }
      },
      {
        "name": "light_id",
        "setting": {
          "min": null,
          "required": null,
          "max": null,
          "advanced": null,
          "group": "Hallway Settings",
          "enum_": null,
          "vtype": "string",
          "ui": null,
          "value": "light-hallway-1",
          "description": "ID of the hallway light"
        }
      },
      {
        "name": "motion_timeout",
        "setting": {
          "max": null,
          "group": "Hallway Settings",
          "enum_": null,
          "required": null,
          "description": "Seconds before auto-off",
          "min": null,
          "ui": null,
          "vtype": "time.seconds",
          "advanced": null,
          "value": 120
        }
      },
      {
        "name": "welcome_message",
        "setting": {
          "advanced": null,
          "max": null,
          "description": "Message to play when the user arrives",
          "required": null,
          "min": null,
          "group": null,
          "vtype": "string",
          "ui": "text",
          "value": "Welcome home!",
          "enum_": null
        }
      },
      {
        "name": "enable_logging",
        "setting": {
          "min": null,
          "advanced": null,
          "group": null,
          "value": true,
          "max": null,
          "ui": "toggle",
          "vtype": "boolean",
          "enum_": null,
          "required": null,
          "description": "Enable verbose logging"
        }
      }
    ],
    "constants": [
      {
        "name": "HUE_BRIDGE_IP",
        "value": "192.168.1.42"
      },
      {
        "name": "API_TOKEN",
        "value": "sk-0ae39..."
      },
      {
        "name": "ENCRYPTION_KEY",
        "value": "kdfj39f..."
      }
    ]

  };
};

// Function to save settings - replace with your API call
const saveSettings = async (data) => {
  // TODO: Replace with actual API call
  // const response = await fetch('YOUR_API_ENDPOINT', {
  //   method: 'PUT',
  //   headers: { 'Content-Type': 'application/json' },
  //   body: JSON.stringify(data)
  // });
  // return await response.json();
  console.log('Saving settings:', data);
  return data;
};

const App = () => {
  const [activeTab, setActiveTab] = useState('settings');
  const [data, setData] = useState({ skill: null, settings: [], constants: [] });
  const [originalData, setOriginalData] = useState({ skill: null, settings: [], constants: [] });
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [collapsedGroups, setCollapsedGroups] = useState({});
  const [errors, setErrors] = useState({});
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    const result = await fetchSettings();
    setData(result);
    setOriginalData(JSON.parse(JSON.stringify(result)));
  };

  const hasChanges = () => {
    return JSON.stringify(data) !== JSON.stringify(originalData);
  };

  const validateSetting = (name, setting) => {
    const { value, required, min, max, vtype } = setting;

    if (required && (!value || value === '')) {
      return 'This field is required';
    }

    if (vtype === 'number' || vtype === 'time.seconds') {
      if (min !== null && value < min) return `Minimum value is ${min}`;
      if (max !== null && value > max) return `Maximum value is ${max}`;
    }

    if (vtype === 'io.ip') {
      const ipRegex = /^(\d{1,3}\.){3}\d{1,3}$/;
      if (!ipRegex.test(value)) return 'Invalid IP address format';
    }

    return null;
  };

  const handleSave = async () => {
    const newErrors = {};

    data.settings.forEach(({ name, setting }) => {
      const error = validateSetting(name, setting);
      if (error) newErrors[name] = error;
    });

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
      return;
    }

    setSaving(true);
    await saveSettings(data);
    setOriginalData(JSON.parse(JSON.stringify(data)));
    setErrors({});
    setSaving(false);
  };

  const handleCancel = () => {
    setData(JSON.parse(JSON.stringify(originalData)));
    setErrors({});
  };

  const updateSetting = (index, newValue) => {
    const newData = { ...data };
    newData.settings[index].setting.value = newValue;
    setData(newData);

    const name = newData.settings[index].name;
    const error = validateSetting(name, newData.settings[index].setting);
    setErrors(prev => ({
      ...prev,
      [name]: error
    }));
  };

  const updateConstant = (index, newValue) => {
    const newData = { ...data };
    newData.constants[index].value = newValue;
    setData(newData);
  };

  const toggleGroup = (group) => {
    setCollapsedGroups(prev => ({
      ...prev,
      [group]: !prev[group]
    }));
  };

  const groupedSettings = () => {
    const groups = {};
    data.settings?.forEach((item, index) => {
      const group = item.setting.group || 'General';
      if (!groups[group]) groups[group] = [];
      groups[group].push({ ...item, index });
    });
    return groups;
  };

  const renderInput = (item, index, isConstant = false) => {
    const { name, setting, value: constValue } = item;
    const { vtype, ui, min, max, enum_, value } = setting || {};
    const displayValue = isConstant ? constValue : value;
    const error = errors[name];

    const updateValue = (newVal) => {
      isConstant ? updateConstant(index, newVal) : updateSetting(index, newVal);
    };

    // Number with slider UI
    if ((vtype === 'number' || vtype === 'time.seconds') && ui === 'slider') {
      return (
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="text-gray-500">{min}</span>
              <span className="text-white font-medium">{displayValue}</span>
              <span className="text-gray-500">{max}</span>
            </div>
            <input
                type="range"
                min={min}
                max={max}
                value={displayValue}
                onChange={(e) => updateValue(Number(e.target.value))}
                className="w-full h-2 bg-gray-800 rounded-lg appearance-none cursor-pointer slider"
            />
          </div>
      );
    }

    // Boolean toggle
    if (vtype === 'boolean') {
      return (
          <button
              onClick={() => updateValue(!displayValue)}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  displayValue ? 'bg-white' : 'bg-gray-800'
              }`}
          >
          <span
              className={`inline-block h-4 w-4 transform rounded-full bg-gray-900 transition-transform ${
                  displayValue ? 'translate-x-6' : 'translate-x-1'
              }`}
          />
          </button>
      );
    }

    // Enum dropdown
    if (vtype === 'enum' && enum_) {
      return (
          <select
              value={displayValue}
              onChange={(e) => updateValue(e.target.value)}
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-gray-500 transition-colors"
          >
            {enum_.map(option => (
                <option key={option} value={option}>{option}</option>
            ))}
          </select>
      );
    }

    // List input (tags)
    if (vtype === 'list') {
      return (
          <div className="space-y-2">
            <div className="flex flex-wrap gap-2">
              {displayValue?.map((item, i) => (
                  <span key={i} className="bg-gray-800 px-3 py-1.5 rounded-full text-sm flex items-center gap-2 border border-gray-700">
                {item}
                    <button
                        onClick={() => updateValue(displayValue.filter((_, idx) => idx !== i))}
                        className="text-gray-500 hover:text-white transition-colors"
                    >
                  <X size={14} />
                </button>
              </span>
              ))}
            </div>
            <input
                type="text"
                placeholder="Type and press Enter"
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && e.target.value) {
                    updateValue([...(displayValue || []), e.target.value]);
                    e.target.value = '';
                  }
                }}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-white placeholder-gray-500 focus:outline-none focus:border-gray-500 transition-colors"
            />
          </div>
      );
    }

    // Password input
    if (ui === 'password') {
      return (
          <input
              type="password"
              value={displayValue}
              onChange={(e) => updateValue(e.target.value)}
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-white placeholder-gray-500 focus:outline-none focus:border-gray-500 transition-colors"
          />
      );
    }

    // Number input
    if (vtype === 'number' || vtype === 'time.seconds') {
      return (
          <input
              type="number"
              min={min}
              max={max}
              value={displayValue}
              onChange={(e) => updateValue(Number(e.target.value))}
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-white placeholder-gray-500 focus:outline-none focus:border-gray-500 transition-colors"
          />
      );
    }

    // Default text input
    return (
        <input
            type="text"
            value={displayValue}
            onChange={(e) => updateValue(e.target.value)}
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2.5 text-white placeholder-gray-500 focus:outline-none focus:border-gray-500 transition-colors"
        />
    );
  };

  const renderSetting = (item, index) => {
    const { name, setting } = item;
    const { description, advanced, required } = setting;
    const error = errors[name];

    if (advanced && !showAdvanced) return null;

    return (
        <div key={name} className="bg-gray-800/50 backdrop-blur rounded-xl p-6 border border-gray-700/50 hover:border-gray-600/50 transition-all">
          <div className="flex justify-between items-start mb-4">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <h3 className="text-white font-medium">{name.replace(/_/g, ' ')}</h3>
                {required && <span className="text-red-400 text-xs">*</span>}
                {advanced && <span className="text-gray-500 text-xs bg-gray-800 px-2 py-1 rounded-md">Advanced</span>}
              </div>
              <p className="text-gray-400 text-sm">{description}</p>
            </div>
          </div>
          {renderInput(item, index)}
          {error && (
              <div className="flex items-center gap-2 mt-3 text-red-400 text-sm bg-red-400/10 px-3 py-2 rounded-lg">
                <AlertCircle size={14} />
                <span>{error}</span>
              </div>
          )}
        </div>
    );
  };

  const groups = groupedSettings();
  const hasAdvanced = data.settings?.some(s => s.setting.advanced);
  const { skill } = data;

  return (
      <div className="min-h-screen bg-gray-950 text-white">
        <style>{`
        .slider::-webkit-slider-thumb {
          appearance: none;
          width: 20px;
          height: 20px;
          border-radius: 50%;
          background: white;
          cursor: pointer;
          box-shadow: 0 2px 8px rgba(0,0,0,0.3);
        }
        .slider::-moz-range-thumb {
          width: 20px;
          height: 20px;
          border-radius: 50%;
          background: white;
          cursor: pointer;
          border: none;
          box-shadow: 0 2px 8px rgba(0,0,0,0.3);
        }
        .grid-pattern {
          background-image: 
            linear-gradient(rgba(255,255,255,0.02) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.02) 1px, transparent 1px);
          background-size: 50px 50px;
        }
      `}</style>

        {/* Header */}
        <div className="border-b border-gray-800/50 grid-pattern">
          <div className="max-w-7xl mx-auto px-6 py-8">
            <h1 className="text-4xl font-bold">Configuration</h1>
            <p className="text-gray-400 mt-2">Manage your skill settings and constants</p>
          </div>
        </div>

        <div className="max-w-7xl mx-auto px-6 py-8">
          {/* Skill Info Card */}
          {skill && (
              <div className="mb-8 bg-gradient-to-br from-gray-800/50 to-gray-900/50 backdrop-blur rounded-2xl p-8 border border-gray-700/50">
                <div className="flex items-start justify-between mb-6">
                  <div className="flex items-center gap-4">
                    <div className="w-16 h-16 bg-gray-800 rounded-xl flex items-center justify-center border border-gray-700">
                      <Package size={32} className="text-white" />
                    </div>
                    <div>
                      <h2 className="text-2xl font-bold text-white">{skill.name}</h2>
                      <p className="text-gray-400 mt-1">{skill.description}</p>
                    </div>
                  </div>
                  <span className="bg-gray-800 text-gray-300 px-4 py-1.5 rounded-full text-sm font-medium border border-gray-700">
                v{skill.version}
              </span>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                  <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-2">
                      <User size={16} />
                      <span className="text-sm font-medium">Author</span>
                    </div>
                    <p className="text-white font-medium">{skill.author}</p>
                  </div>

                  <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-2">
                      <Code size={16} />
                      <span className="text-sm font-medium">Entry Point</span>
                    </div>
                    <p className="text-white font-medium font-mono text-sm">{skill.entry}</p>
                  </div>

                  <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-2">
                      <Zap size={16} />
                      <span className="text-sm font-medium">Capabilities</span>
                    </div>
                    <p className="text-white font-medium">{skill.capabilities?.length || 0}</p>
                  </div>

                  <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-800">
                    <div className="flex items-center gap-2 text-gray-400 mb-2">
                      <Shield size={16} />
                      <span className="text-sm font-medium">Permissions</span>
                    </div>
                    <p className="text-white font-medium">{skill.permissions?.length || 0}</p>
                  </div>
                </div>

                {skill.capabilities && skill.capabilities.length > 0 && (
                    <div className="mt-4 pt-4 border-t border-gray-800">
                      <p className="text-gray-400 text-sm mb-2">Capabilities:</p>
                      <div className="flex flex-wrap gap-2">
                        {skill.capabilities.map((cap, i) => (
                            <span key={i} className="bg-gray-800 text-gray-300 px-3 py-1 rounded-md text-xs font-mono border border-gray-700">
                      {cap}
                    </span>
                        ))}
                      </div>
                    </div>
                )}

                {skill.permissions && skill.permissions.length > 0 && (
                    <div className="mt-4">
                      <p className="text-gray-400 text-sm mb-2">Permissions:</p>
                      <div className="flex flex-wrap gap-2">
                        {skill.permissions.map((perm, i) => (
                            <span key={i} className="bg-gray-800 text-gray-300 px-3 py-1 rounded-md text-xs font-mono border border-gray-700">
                      {perm}
                    </span>
                        ))}
                      </div>
                    </div>
                )}
              </div>
          )}

          {/* Tabs */}
          <div className="flex gap-4 mb-8">
            <button
                onClick={() => setActiveTab('settings')}
                className={`px-6 py-3 rounded-xl transition-all flex items-center gap-2 font-medium ${
                    activeTab === 'settings'
                        ? 'bg-white text-gray-900'
                        : 'bg-gray-800/50 text-gray-400 hover:text-white hover:bg-gray-800 border border-gray-700/50'
                }`}
            >
              <Settings size={18} />
              Settings
            </button>
            <button
                onClick={() => setActiveTab('constants')}
                className={`px-6 py-3 rounded-xl transition-all flex items-center gap-2 font-medium ${
                    activeTab === 'constants'
                        ? 'bg-white text-gray-900'
                        : 'bg-gray-800/50 text-gray-400 hover:text-white hover:bg-gray-800 border border-gray-700/50'
                }`}
            >
              <Database size={18} />
              Constants
            </button>
          </div>

          {/* Content */}
          {activeTab === 'settings' && (
              <div className="space-y-8">
                {hasAdvanced && (
                    <div className="flex justify-end">
                      <button
                          onClick={() => setShowAdvanced(!showAdvanced)}
                          className="text-gray-400 hover:text-white text-sm transition-colors px-4 py-2 rounded-lg bg-gray-800/50 border border-gray-700/50"
                      >
                        {showAdvanced ? 'Hide' : 'Show'} Advanced Settings
                      </button>
                    </div>
                )}

                {Object.entries(groups).map(([groupName, items]) => (
                    <div key={groupName}>
                      <button
                          onClick={() => toggleGroup(groupName)}
                          className="flex items-center gap-3 mb-6 text-xl font-semibold hover:text-gray-300 transition-colors"
                      >
                        {collapsedGroups[groupName] ? <ChevronRight size={24} /> : <ChevronDown size={24} />}
                        {groupName}
                      </button>
                      {!collapsedGroups[groupName] && (
                          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            {items.map(item => renderSetting(item, item.index))}
                          </div>
                      )}
                    </div>
                ))}
              </div>
          )}

          {activeTab === 'constants' && (
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {data.constants?.map((constant, index) => (
                    <div key={constant.name} className="bg-gray-800/50 backdrop-blur rounded-xl p-6 border border-gray-700/50 hover:border-gray-600/50 transition-all">
                      <h3 className="text-white font-medium mb-4">{constant.name}</h3>
                      <input
                          type="text"
                          value={constant.value}
                          onChange={(e) => updateConstant(index, e.target.value)}
                          className="w-full bg-gray-900/50 border border-gray-700 rounded-lg px-4 py-2.5 text-white font-mono text-sm focus:outline-none focus:border-gray-500 transition-colors"
                      />
                    </div>
                ))}
              </div>
          )}
        </div>

        {/* Action Bar */}
        {hasChanges() && (
            <div className="fixed bottom-0 left-0 right-0 bg-gray-900/95 backdrop-blur-xl border-t border-gray-800 py-4">
              <div className="max-w-7xl mx-auto px-6 flex items-center justify-between">
                <span className="text-gray-400 text-sm">You have unsaved changes</span>
                <div className="flex gap-3">
                  <button
                      onClick={handleCancel}
                      className="px-6 py-2.5 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors flex items-center gap-2 border border-gray-700"
                  >
                    <X size={18} />
                    Cancel
                  </button>
                  <button
                      onClick={handleSave}
                      disabled={saving}
                      className="px-6 py-2.5 bg-white text-gray-900 hover:bg-gray-200 rounded-lg font-medium transition-colors flex items-center gap-2 disabled:opacity-50"
                  >
                    <Save size={18} />
                    {saving ? 'Saving...' : 'Save All'}
                  </button>
                </div>
              </div>
            </div>
        )}
      </div>
  );
};

export default App;
