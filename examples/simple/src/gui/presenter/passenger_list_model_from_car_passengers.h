// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/passenger_dto.h"
#include "simple_example_presenter_export.h"
#include <QAbstractListModel>

using namespace Simple::Contracts::DTO::Passenger;

namespace Simple::Presenter
{
class SIMPLE_EXAMPLE_PRESENTER_EXPORT PassengerListModelFromCarPassengers : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int carId READ carId WRITE setCarId RESET resetCarId NOTIFY carIdChanged FINAL)

  public:
    enum Roles
    {

        IdRole = Qt::UserRole + 0,
        UuidRole = Qt::UserRole + 1,
        CreationDateRole = Qt::UserRole + 2,
        UpdateDateRole = Qt::UserRole + 3,
        NameRole = Qt::UserRole + 4
    };
    Q_ENUM(Roles)

    explicit PassengerListModelFromCarPassengers(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    int carId() const;
    void setCarId(int newCarId);
    void resetCarId();

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

    QHash<int, QByteArray> roleNames() const override;

  Q_SIGNALS:
    void carIdChanged();

  private:
    void populate();

    QList<PassengerDTO> m_passengerList;
    QList<int> m_passengerIdList;
    int m_carId = 0;
};

} // namespace Simple::Presenter