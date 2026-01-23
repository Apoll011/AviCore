import React, { useState, useEffect } from 'react';
import { Save, X, ChevronDown, ChevronRight, Settings, Database, AlertCircle, Package, User, Code, Shield, Zap } from 'lucide-react';
import './skill_input.css'

const fetchSettings = async () => {
    // TODO: Replace with actual API call
    // const response = await fetch('YOUR_API_ENDPOINT');
    // return await response.json();

    // Example data structure (remove when implementing API)
    return {
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

const SettingsPage = () => {
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
                            <h3 className="text-white font-medium capitalize">{name.replace(/_/g, ' ')}</h3>
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

    return (
        <>
            <div className="mx-auto">
                {/* Tabs */}
                <div className="flex items-center gap-3 mb-8 border-b border-gray-800">
                    <button
                        onClick={() => setActiveTab('settings')}
                        className={`px-5 py-3 transition-all flex items-center gap-2 font-medium text-sm border-b-2 ${
                            activeTab === 'settings'
                                ? 'border-blue-500 text-white'
                                : 'border-transparent text-gray-400 hover:text-gray-300 hover:bg-gray-800/30'
                        }`}
                    >
                        <Settings size={16} />
                        Settings
                    </button>
                    <button
                        onClick={() => setActiveTab('constants')}
                        className={`px-5 py-3 transition-all flex items-center gap-2 font-medium text-sm border-b-2 ${
                            activeTab === 'constants'
                                ? 'border-blue-500 text-white'
                                : 'border-transparent text-gray-400 hover:text-gray-300 hover:bg-gray-800/30'
                        }`}
                    >
                        <Database size={16} />
                        Constants
                    </button>
                    {hasAdvanced && activeTab === 'settings' && (
                        <div className="ml-auto pb-2">
                            <button
                                onClick={() => setShowAdvanced(!showAdvanced)}
                                className="text-gray-400 hover:text-white text-xs transition-colors px-4 py-2 rounded-lg bg-gray-800/50 hover:bg-gray-800 border border-gray-700/50 hover:border-gray-600"
                            >
                                {showAdvanced ? 'Hide' : 'Show'} Advanced
                            </button>
                        </div>
                    )}
                </div>
                {/* Content */}
                {activeTab === 'settings' && (
                    <div className="space-y-8">
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
        </>
    );
};

export default SettingsPage;