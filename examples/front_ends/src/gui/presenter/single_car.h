// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_presenter_export.h"
#include <QObject>

#include <QDateTime>

#include <QDateTime>

#include <QUuid>

namespace FrontEnds::Presenter
{
class FRONT_ENDS_EXAMPLE_PRESENTER_EXPORT SingleCar : public QObject
{
    Q_OBJECT
    Q_PROPERTY(int itemId READ id WRITE setId RESET resetId NOTIFY idChanged FINAL)

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid NOTIFY uuidChanged FINAL)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate NOTIFY creationDateChanged FINAL)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate NOTIFY updateDateChanged FINAL)
    Q_PROPERTY(QString content READ content WRITE setContent NOTIFY contentChanged FINAL)

  public:
    explicit SingleCar(QObject *parent = nullptr);

    int id() const;
    void setId(int newId);
    void resetId();

    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

    QDateTime creationDate() const;
    void setCreationDate(const QDateTime &newCreationDate);

    QDateTime updateDate() const;
    void setUpdateDate(const QDateTime &newUpdateDate);

    QString content() const;
    void setContent(const QString &newContent);

  signals:

    void idChanged();

    void uuidChanged();
    void creationDateChanged();
    void updateDateChanged();
    void contentChanged();

  private:
    int m_id;

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_content;
};

} // namespace FrontEnds::Presenter