// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once

#include "qleany/qleany_global.h"
#include <QDateTime>
#include <QHash>
#include <QMutexLocker>
#include <QVariant>
#include <QtCore/QMetaObject>
#include <QtCore/QMetaProperty>

namespace Qleany::Tools::AutoMapper
{
class QLEANY_EXPORT AutoMapper
{
  public:
    template <class SourceType, class DestinationType> static void registerMapping(bool reverseConversion = false)
    {
        QMutexLocker locker(&mutex);

        qRegisterMetaType<SourceType>();
        qRegisterMetaType<DestinationType>();

        getSiblingFunctions[QMetaType::fromType<SourceType>()] = []() {
            return QVariant::fromValue(DestinationType());
        };

        getSiblingFunctions[QMetaType::fromType<QList<SourceType>>()] = []() {
            return QVariant::fromValue(DestinationType());
        };

        writerHash[QMetaType::fromType<SourceType>()] = [&](const QMetaProperty &destinationProperty,
                                                            void *gadgetPointer, const QVariant &sourceValue) {
            const QVariant &customDestinationObject = AutoMapper::implMap(sourceValue);
            DestinationType destinationValue = customDestinationObject.value<DestinationType>();

            return destinationProperty.writeOnGadget(gadgetPointer, QVariant::fromValue(destinationValue));
        };

        writerForListHash[QMetaType::fromType<QList<SourceType>>()] =
            [](const QMetaProperty &destinationProperty, void *gadgetPointer, const QList<QVariant> &sourceList) {
                QList<DestinationType> destinationList;

                for (const QVariant &value : sourceList)
                {
                    const QVariant &customDestinationObject = AutoMapper::implMap(value);
                    destinationList.append(customDestinationObject.value<DestinationType>());
                }

                auto variant = QVariant::fromValue(destinationList);
                return destinationProperty.writeOnGadget(gadgetPointer, variant);
            };

        if (reverseConversion)
        {
            getSiblingFunctions[QMetaType::fromType<DestinationType>()] = []() {
                return QVariant::fromValue(SourceType());
            };

            getSiblingFunctions[QMetaType::fromType<QList<DestinationType>>()] = []() {
                return QVariant::fromValue(SourceType());
            };

            writerHash[QMetaType::fromType<DestinationType>()] = [](const QMetaProperty &destinationProperty,
                                                                    void *gadgetPointer, const QVariant &sourceValue) {
                const QVariant &customDestinationObject = AutoMapper::implMap(sourceValue);
                SourceType destinationValue = customDestinationObject.value<SourceType>();

                return destinationProperty.writeOnGadget(gadgetPointer, QVariant::fromValue(destinationValue));
            };

            writerForListHash[QMetaType::fromType<QList<DestinationType>>()] =
                [](const QMetaProperty &destinationProperty, void *gadgetPointer, const QList<QVariant> &sourceList) {
                    QList<SourceType> destinationList;

                    for (const QVariant &value : sourceList)
                    {
                        const QVariant &customDestinationObject = AutoMapper::implMap(value);
                        destinationList.append(customDestinationObject.value<SourceType>());
                    }
                    auto variant = QVariant::fromValue(destinationList);
                    return destinationProperty.writeOnGadget(gadgetPointer, variant);
                };
        }

        qDebug() << "Entries after registering: " << AutoMapper::AutoMapper::getSiblingFunctions.count();
    }

    template <class SourceType, class DestinationType> static DestinationType map(const SourceType &sourceObject)
    {
        const QVariant &destinationVariant = implMap(QVariant::fromValue(sourceObject));

        return destinationVariant.value<DestinationType>();
    }

    template <class SourceType, class DestinationType> static DestinationType map(SourceType &sourceObject)
    {
        const QVariant &destinationVariant = implMap(QVariant::fromValue(sourceObject));

        return destinationVariant.value<DestinationType>();
    }

  private:
    static QVariant implMap(const QVariant &source);

  private:
    static QHash<QMetaType, std::function<QVariant()>> getSiblingFunctions;
    static QHash<QMetaType, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer,
                                               const QVariant &sourceValue)>>
        writerHash;
    static QHash<QMetaType, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer,
                                               const QList<QVariant> &sourceList)>>
        writerForListHash;

    static QMutex mutex;
};
} // namespace Qleany::Tools::AutoMapper
